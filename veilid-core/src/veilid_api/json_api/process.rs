use super::*;
use futures_util::FutureExt;

pub fn to_json_api_result<T: Clone + fmt::Debug + JsonSchema>(
    r: VeilidAPIResult<T>,
) -> json_api::ApiResult<T> {
    match r {
        Err(e) => json_api::ApiResult::Err { error: e },
        Ok(v) => json_api::ApiResult::Ok { value: v },
    }
}

pub fn to_json_api_result_with_string<T: Clone + fmt::Debug>(
    r: VeilidAPIResult<T>,
) -> json_api::ApiResultWithString<T> {
    match r {
        Err(e) => json_api::ApiResultWithString::Err { error: e },
        Ok(v) => json_api::ApiResultWithString::Ok { value: v },
    }
}

pub fn to_json_api_result_with_vec_string<T: Clone + fmt::Debug>(
    r: VeilidAPIResult<T>,
) -> json_api::ApiResultWithVecString<T> {
    match r {
        Err(e) => json_api::ApiResultWithVecString::Err { error: e },
        Ok(v) => json_api::ApiResultWithVecString::Ok { value: v },
    }
}

pub fn to_json_api_result_with_vec_u8(r: VeilidAPIResult<Vec<u8>>) -> json_api::ApiResultWithVecU8 {
    match r {
        Err(e) => json_api::ApiResultWithVecU8::Err { error: e },
        Ok(v) => json_api::ApiResultWithVecU8::Ok { value: v },
    }
}

pub fn to_json_api_result_with_vec_vec_u8(
    r: VeilidAPIResult<Vec<Vec<u8>>>,
) -> json_api::ApiResultWithVecVecU8 {
    match r {
        Err(e) => json_api::ApiResultWithVecVecU8::Err { error: e },
        Ok(v) => json_api::ApiResultWithVecVecU8::Ok {
            value: v.into_iter().map(|v| VecU8 { value: v }).collect(),
        },
    }
}

struct JsonRequestProcessorInner {
    routing_contexts: BTreeMap<u32, RoutingContext>,
    table_dbs: BTreeMap<u32, TableDB>,
    table_db_transactions: BTreeMap<u32, TableDBTransaction>,
    crypto_systems: BTreeMap<u32, CryptoSystemVersion>,
}

#[derive(Clone)]
pub struct JsonRequestProcessor {
    api: VeilidAPI,
    inner: Arc<Mutex<JsonRequestProcessorInner>>,
}

impl JsonRequestProcessor {
    pub fn new(api: VeilidAPI) -> Self {
        Self {
            api,
            inner: Arc::new(Mutex::new(JsonRequestProcessorInner {
                routing_contexts: Default::default(),
                table_dbs: Default::default(),
                table_db_transactions: Default::default(),
                crypto_systems: Default::default(),
            })),
        }
    }

    // Routing Context
    fn add_routing_context(&self, routing_context: RoutingContext) -> u32 {
        let mut inner = self.inner.lock();
        let mut next_id: u32 = 1;
        while inner.routing_contexts.contains_key(&next_id) {
            next_id += 1;
        }
        inner.routing_contexts.insert(next_id, routing_context);
        next_id
    }
    fn lookup_routing_context(&self, id: u32, rc_id: u32) -> Result<RoutingContext, Response> {
        let inner = self.inner.lock();
        let Some(routing_context) = inner.routing_contexts.get(&rc_id).cloned() else {
            return Err(Response {
                id,
                op: ResponseOp::RoutingContext(Box::new(RoutingContextResponse {
                    rc_id,
                    rc_op: RoutingContextResponseOp::InvalidId,
                })),
            });
        };
        Ok(routing_context)
    }
    fn release_routing_context(&self, id: u32) -> i32 {
        let mut inner = self.inner.lock();
        if inner.routing_contexts.remove(&id).is_none() {
            return 0;
        }
        1
    }

    // TableDB
    fn add_table_db(&self, table_db: TableDB) -> u32 {
        let mut inner = self.inner.lock();
        let mut next_id: u32 = 1;
        while inner.table_dbs.contains_key(&next_id) {
            next_id += 1;
        }
        inner.table_dbs.insert(next_id, table_db);
        next_id
    }
    fn lookup_table_db(&self, id: u32, db_id: u32) -> Result<TableDB, Response> {
        let inner = self.inner.lock();
        let Some(table_db) = inner.table_dbs.get(&db_id).cloned() else {
            return Err(Response {
                id,
                op: ResponseOp::TableDb(TableDbResponse {
                    db_id,
                    db_op: TableDbResponseOp::InvalidId,
                }),
            });
        };
        Ok(table_db)
    }
    fn release_table_db(&self, id: u32) -> i32 {
        let mut inner = self.inner.lock();
        if inner.table_dbs.remove(&id).is_none() {
            return 0;
        }
        1
    }

    // TableDBTransaction
    fn add_table_db_transaction(&self, tdbt: TableDBTransaction) -> u32 {
        let mut inner = self.inner.lock();
        let mut next_id: u32 = 1;
        while inner.table_db_transactions.contains_key(&next_id) {
            next_id += 1;
        }
        inner.table_db_transactions.insert(next_id, tdbt);
        next_id
    }
    fn lookup_table_db_transaction(
        &self,
        id: u32,
        tx_id: u32,
    ) -> Result<TableDBTransaction, Response> {
        let inner = self.inner.lock();
        let Some(table_db_transaction) = inner.table_db_transactions.get(&tx_id).cloned() else {
            return Err(Response {
                id,
                op: ResponseOp::TableDbTransaction(TableDbTransactionResponse {
                    tx_id,
                    tx_op: TableDbTransactionResponseOp::InvalidId,
                }),
            });
        };
        Ok(table_db_transaction)
    }
    fn release_table_db_transaction(&self, id: u32) -> i32 {
        let mut inner = self.inner.lock();
        if inner.table_db_transactions.remove(&id).is_none() {
            return 0;
        }
        1
    }

    // CryptoSystem
    fn add_crypto_system(&self, csv: CryptoSystemVersion) -> u32 {
        let mut inner = self.inner.lock();
        let mut next_id: u32 = 1;
        while inner.crypto_systems.contains_key(&next_id) {
            next_id += 1;
        }
        inner.crypto_systems.insert(next_id, csv);
        next_id
    }
    fn lookup_crypto_system(&self, id: u32, cs_id: u32) -> Result<CryptoSystemVersion, Response> {
        let inner = self.inner.lock();
        let Some(crypto_system) = inner.crypto_systems.get(&cs_id).cloned() else {
            return Err(Response {
                id,
                op: ResponseOp::CryptoSystem(CryptoSystemResponse {
                    cs_id,
                    cs_op: CryptoSystemResponseOp::InvalidId,
                }),
            });
        };
        Ok(crypto_system)
    }
    fn release_crypto_system(&self, id: u32) -> i32 {
        let mut inner = self.inner.lock();
        if inner.crypto_systems.remove(&id).is_none() {
            return 0;
        }
        1
    }

    // Target

    // Parse target
    async fn parse_target(&self, s: String) -> VeilidAPIResult<Target> {
        // Is this a route id?
        if let Ok(rrid) = RouteId::from_str(&s) {
            let routing_table = self.api.routing_table()?;
            let rss = routing_table.route_spec_store();

            // Is this a valid remote route id? (can't target allocated routes)
            if rss.is_route_id_remote(&rrid) {
                return Ok(Target::PrivateRoute(rrid));
            }
        }

        // Is this a node id?
        if let Ok(nid) = TypedKey::from_str(&s) {
            return Ok(Target::NodeId(nid));
        }

        Err(VeilidAPIError::parse_error("Unable to parse as target", s))
    }

    //////////////////////////////////////////////////////////////////////////////////////

    pub async fn process_routing_context_request(
        &self,
        routing_context: RoutingContext,
        rcr: RoutingContextRequest,
    ) -> RoutingContextResponse {
        let rc_op = match rcr.rc_op {
            RoutingContextRequestOp::Release => {
                self.release_routing_context(rcr.rc_id);
                RoutingContextResponseOp::Release {}
            }
            RoutingContextRequestOp::WithDefaultSafety => {
                RoutingContextResponseOp::WithDefaultSafety {
                    result: to_json_api_result(
                        routing_context
                            .clone()
                            .with_default_safety()
                            .map(|new_rc| self.add_routing_context(new_rc)),
                    ),
                }
            }
            RoutingContextRequestOp::WithSafety { safety_selection } => {
                RoutingContextResponseOp::WithSafety {
                    result: to_json_api_result(
                        routing_context
                            .clone()
                            .with_safety(safety_selection)
                            .map(|new_rc| self.add_routing_context(new_rc)),
                    ),
                }
            }
            RoutingContextRequestOp::WithSequencing { sequencing } => {
                RoutingContextResponseOp::WithSequencing {
                    value: self
                        .add_routing_context(routing_context.clone().with_sequencing(sequencing)),
                }
            }
            RoutingContextRequestOp::Safety => RoutingContextResponseOp::Safety {
                value: routing_context.safety(),
            },
            RoutingContextRequestOp::AppCall { target, message } => {
                RoutingContextResponseOp::AppCall {
                    result: to_json_api_result_with_vec_u8(
                        self.parse_target(target)
                            .then(|tr| async { routing_context.app_call(tr?, message).await })
                            .await,
                    ),
                }
            }
            RoutingContextRequestOp::AppMessage { target, message } => {
                RoutingContextResponseOp::AppMessage {
                    result: to_json_api_result(
                        self.parse_target(target)
                            .then(|tr| async { routing_context.app_message(tr?, message).await })
                            .await,
                    ),
                }
            }
            RoutingContextRequestOp::CreateDhtRecord { schema, kind } => {
                RoutingContextResponseOp::CreateDhtRecord {
                    result: to_json_api_result(
                        routing_context
                            .create_dht_record(schema, kind)
                            .await
                            .map(Box::new),
                    ),
                }
            }
            RoutingContextRequestOp::OpenDhtRecord { key, writer } => {
                RoutingContextResponseOp::OpenDhtRecord {
                    result: to_json_api_result(
                        routing_context
                            .open_dht_record(key, writer)
                            .await
                            .map(Box::new),
                    ),
                }
            }
            RoutingContextRequestOp::CloseDhtRecord { key } => {
                RoutingContextResponseOp::CloseDhtRecord {
                    result: to_json_api_result(routing_context.close_dht_record(key).await),
                }
            }
            RoutingContextRequestOp::DeleteDhtRecord { key } => {
                RoutingContextResponseOp::DeleteDhtRecord {
                    result: to_json_api_result(routing_context.delete_dht_record(key).await),
                }
            }
            RoutingContextRequestOp::GetDhtValue {
                key,
                subkey,
                force_refresh,
            } => RoutingContextResponseOp::GetDhtValue {
                result: to_json_api_result(
                    routing_context
                        .get_dht_value(key, subkey, force_refresh)
                        .await,
                ),
            },
            RoutingContextRequestOp::SetDhtValue { key, subkey, data } => {
                RoutingContextResponseOp::SetDhtValue {
                    result: to_json_api_result(
                        routing_context.set_dht_value(key, subkey, data).await,
                    ),
                }
            }
            RoutingContextRequestOp::WatchDhtValues {
                key,
                subkeys,
                expiration,
                count,
            } => RoutingContextResponseOp::WatchDhtValues {
                result: to_json_api_result(
                    routing_context
                        .watch_dht_values(key, subkeys, expiration, count)
                        .await,
                ),
            },
            RoutingContextRequestOp::CancelDhtWatch { key, subkeys } => {
                RoutingContextResponseOp::CancelDhtWatch {
                    result: to_json_api_result(
                        routing_context.cancel_dht_watch(key, subkeys).await,
                    ),
                }
            }
        };
        RoutingContextResponse {
            rc_id: rcr.rc_id,
            rc_op,
        }
    }

    pub async fn process_table_db_request(
        &self,
        table_db: TableDB,
        tdr: TableDbRequest,
    ) -> TableDbResponse {
        let db_op = match tdr.db_op {
            TableDbRequestOp::Release => {
                self.release_table_db(tdr.db_id);
                TableDbResponseOp::Release {}
            }
            TableDbRequestOp::GetColumnCount => TableDbResponseOp::GetColumnCount {
                result: to_json_api_result(table_db.get_column_count()),
            },
            TableDbRequestOp::GetKeys { col } => TableDbResponseOp::GetKeys {
                result: to_json_api_result_with_vec_vec_u8(table_db.get_keys(col).await),
            },
            TableDbRequestOp::Transact => TableDbResponseOp::Transact {
                value: self.add_table_db_transaction(table_db.transact()),
            },
            TableDbRequestOp::Store { col, key, value } => TableDbResponseOp::Store {
                result: to_json_api_result(table_db.store(col, &key, &value).await),
            },
            TableDbRequestOp::Load { col, key } => TableDbResponseOp::Load {
                result: to_json_api_result(
                    table_db
                        .load(col, &key)
                        .await
                        .map(|vopt| vopt.map(|v| VecU8 { value: v })),
                ),
            },
            TableDbRequestOp::Delete { col, key } => TableDbResponseOp::Delete {
                result: to_json_api_result(
                    table_db
                        .delete(col, &key)
                        .await
                        .map(|vopt| vopt.map(|v| VecU8 { value: v })),
                ),
            },
        };
        TableDbResponse {
            db_id: tdr.db_id,
            db_op,
        }
    }

    pub async fn process_table_db_transaction_request(
        &self,
        table_db_transaction: TableDBTransaction,
        tdtr: TableDbTransactionRequest,
    ) -> TableDbTransactionResponse {
        let tx_op = match tdtr.tx_op {
            TableDbTransactionRequestOp::Commit => TableDbTransactionResponseOp::Commit {
                result: to_json_api_result(table_db_transaction.commit().await.map(|_| {
                    self.release_table_db_transaction(tdtr.tx_id);
                })),
            },
            TableDbTransactionRequestOp::Rollback => {
                table_db_transaction.rollback();
                self.release_table_db_transaction(tdtr.tx_id);
                TableDbTransactionResponseOp::Rollback {}
            }
            TableDbTransactionRequestOp::Store { col, key, value } => {
                TableDbTransactionResponseOp::Store {
                    result: to_json_api_result(table_db_transaction.store(col, &key, &value)),
                }
            }

            TableDbTransactionRequestOp::Delete { col, key } => {
                TableDbTransactionResponseOp::Delete {
                    result: to_json_api_result(table_db_transaction.delete(col, &key)),
                }
            }
        };
        TableDbTransactionResponse {
            tx_id: tdtr.tx_id,
            tx_op,
        }
    }

    pub async fn process_crypto_system_request(
        &self,
        csv: CryptoSystemVersion,
        csr: CryptoSystemRequest,
    ) -> CryptoSystemResponse {
        let cs_op = match csr.cs_op {
            CryptoSystemRequestOp::Release => {
                self.release_crypto_system(csr.cs_id);
                CryptoSystemResponseOp::Release {}
            }
            CryptoSystemRequestOp::CachedDh { key, secret } => CryptoSystemResponseOp::CachedDh {
                result: to_json_api_result_with_string(csv.cached_dh(&key, &secret)),
            },
            CryptoSystemRequestOp::ComputeDh { key, secret } => CryptoSystemResponseOp::ComputeDh {
                result: to_json_api_result_with_string(csv.compute_dh(&key, &secret)),
            },
            CryptoSystemRequestOp::RandomBytes { len } => CryptoSystemResponseOp::RandomBytes {
                value: csv.random_bytes(len),
            },
            CryptoSystemRequestOp::DefaultSaltLength => CryptoSystemResponseOp::DefaultSaltLength {
                value: csv.default_salt_length(),
            },
            CryptoSystemRequestOp::HashPassword { password, salt } => {
                CryptoSystemResponseOp::HashPassword {
                    result: to_json_api_result(csv.hash_password(&password, &salt)),
                }
            }
            CryptoSystemRequestOp::VerifyPassword {
                password,
                password_hash,
            } => CryptoSystemResponseOp::VerifyPassword {
                result: to_json_api_result(csv.verify_password(&password, &password_hash)),
            },
            CryptoSystemRequestOp::DeriveSharedSecret { password, salt } => {
                CryptoSystemResponseOp::DeriveSharedSecret {
                    result: to_json_api_result_with_string(
                        csv.derive_shared_secret(&password, &salt),
                    ),
                }
            }
            CryptoSystemRequestOp::RandomNonce => CryptoSystemResponseOp::RandomNonce {
                value: csv.random_nonce(),
            },
            CryptoSystemRequestOp::RandomSharedSecret => {
                CryptoSystemResponseOp::RandomSharedSecret {
                    value: csv.random_shared_secret(),
                }
            }
            CryptoSystemRequestOp::GenerateKeyPair => CryptoSystemResponseOp::GenerateKeyPair {
                value: csv.generate_keypair(),
            },
            CryptoSystemRequestOp::GenerateHash { data } => CryptoSystemResponseOp::GenerateHash {
                value: csv.generate_hash(&data),
            },
            CryptoSystemRequestOp::ValidateKeyPair { key, secret } => {
                CryptoSystemResponseOp::ValidateKeyPair {
                    value: csv.validate_keypair(&key, &secret),
                }
            }
            CryptoSystemRequestOp::ValidateHash { data, hash_digest } => {
                CryptoSystemResponseOp::ValidateHash {
                    value: csv.validate_hash(&data, &hash_digest),
                }
            }
            CryptoSystemRequestOp::Distance { key1, key2 } => CryptoSystemResponseOp::Distance {
                value: csv.distance(&key1, &key2),
            },
            CryptoSystemRequestOp::Sign { key, secret, data } => CryptoSystemResponseOp::Sign {
                result: to_json_api_result_with_string(csv.sign(&key, &secret, &data)),
            },
            CryptoSystemRequestOp::Verify { key, data, secret } => CryptoSystemResponseOp::Verify {
                result: to_json_api_result(csv.verify(&key, &data, &secret)),
            },
            CryptoSystemRequestOp::AeadOverhead => CryptoSystemResponseOp::AeadOverhead {
                value: csv.aead_overhead() as u32,
            },
            CryptoSystemRequestOp::DecryptAead {
                body,
                nonce,
                shared_secret,
                associated_data,
            } => CryptoSystemResponseOp::DecryptAead {
                result: to_json_api_result_with_vec_u8(csv.decrypt_aead(
                    &body,
                    &nonce,
                    &shared_secret,
                    associated_data.as_deref(),
                )),
            },
            CryptoSystemRequestOp::EncryptAead {
                body,
                nonce,
                shared_secret,
                associated_data,
            } => CryptoSystemResponseOp::EncryptAead {
                result: to_json_api_result_with_vec_u8(csv.encrypt_aead(
                    &body,
                    &nonce,
                    &shared_secret,
                    associated_data.as_deref(),
                )),
            },
            CryptoSystemRequestOp::CryptNoAuth {
                body,
                nonce,
                shared_secret,
            } => CryptoSystemResponseOp::CryptNoAuth {
                value: csv.crypt_no_auth_unaligned(&body, &nonce, &shared_secret),
            },
        };
        CryptoSystemResponse {
            cs_id: csr.cs_id,
            cs_op,
        }
    }

    pub async fn process_request(self, request: Request) -> Response {
        let id = request.id;

        let op = match request.op {
            RequestOp::Control { args: _args } => ResponseOp::Control {
                result: to_json_api_result(VeilidAPIResult::Err(VeilidAPIError::unimplemented(
                    "control should be handled by veilid-core host application",
                ))),
            },
            RequestOp::GetState => ResponseOp::GetState {
                result: to_json_api_result(self.api.get_state().await.map(Box::new)),
            },
            RequestOp::Attach => ResponseOp::Attach {
                result: to_json_api_result(self.api.attach().await),
            },
            RequestOp::Detach => ResponseOp::Detach {
                result: to_json_api_result(self.api.detach().await),
            },
            RequestOp::NewPrivateRoute => ResponseOp::NewPrivateRoute {
                result: to_json_api_result(self.api.new_private_route().await.map(|r| {
                    NewPrivateRouteResult {
                        route_id: r.0,
                        blob: r.1,
                    }
                })),
            },
            RequestOp::NewCustomPrivateRoute {
                kinds,
                stability,
                sequencing,
            } => ResponseOp::NewCustomPrivateRoute {
                result: to_json_api_result(
                    self.api
                        .new_custom_private_route(&kinds, stability, sequencing)
                        .await
                        .map(|r| NewPrivateRouteResult {
                            route_id: r.0,
                            blob: r.1,
                        }),
                ),
            },
            RequestOp::ImportRemotePrivateRoute { blob } => ResponseOp::ImportRemotePrivateRoute {
                result: to_json_api_result_with_string(self.api.import_remote_private_route(blob)),
            },
            RequestOp::ReleasePrivateRoute { route_id } => ResponseOp::ReleasePrivateRoute {
                result: to_json_api_result(self.api.release_private_route(route_id)),
            },
            RequestOp::AppCallReply { call_id, message } => ResponseOp::AppCallReply {
                result: to_json_api_result(self.api.app_call_reply(call_id, message).await),
            },
            RequestOp::NewRoutingContext => ResponseOp::NewRoutingContext {
                result: to_json_api_result(
                    self.api
                        .routing_context()
                        .map(|rc| self.add_routing_context(rc)),
                ),
            },
            RequestOp::RoutingContext(rcr) => {
                let routing_context = match self.lookup_routing_context(id, rcr.rc_id) {
                    Ok(v) => v,
                    Err(e) => return e,
                };
                ResponseOp::RoutingContext(Box::new(
                    self.process_routing_context_request(routing_context, rcr)
                        .await,
                ))
            }
            RequestOp::OpenTableDb { name, column_count } => {
                let table_store = match self.api.table_store() {
                    Ok(v) => v,
                    Err(e) => {
                        return Response {
                            id,
                            op: ResponseOp::OpenTableDb {
                                result: to_json_api_result(Err(e)),
                            },
                        }
                    }
                };
                ResponseOp::OpenTableDb {
                    result: to_json_api_result(
                        table_store
                            .open(&name, column_count)
                            .await
                            .map(|table_db| self.add_table_db(table_db)),
                    ),
                }
            }
            RequestOp::DeleteTableDb { name } => {
                let table_store = match self.api.table_store() {
                    Ok(v) => v,
                    Err(e) => {
                        return Response {
                            id,
                            op: ResponseOp::OpenTableDb {
                                result: to_json_api_result(Err(e)),
                            },
                        }
                    }
                };
                ResponseOp::DeleteTableDb {
                    result: to_json_api_result(table_store.delete(&name).await),
                }
            }
            RequestOp::TableDb(tdr) => {
                let table_db = match self.lookup_table_db(id, tdr.db_id) {
                    Ok(v) => v,
                    Err(e) => return e,
                };
                ResponseOp::TableDb(self.process_table_db_request(table_db, tdr).await)
            }
            RequestOp::TableDbTransaction(tdtr) => {
                let table_db_transaction = match self.lookup_table_db_transaction(id, tdtr.tx_id) {
                    Ok(v) => v,
                    Err(e) => return e,
                };
                ResponseOp::TableDbTransaction(
                    self.process_table_db_transaction_request(table_db_transaction, tdtr)
                        .await,
                )
            }
            RequestOp::GetCryptoSystem { kind } => {
                let crypto = match self.api.crypto() {
                    Ok(v) => v,
                    Err(e) => {
                        return Response {
                            id,
                            op: ResponseOp::GetCryptoSystem {
                                result: to_json_api_result(Err(e)),
                            },
                        }
                    }
                };
                ResponseOp::GetCryptoSystem {
                    result: to_json_api_result(
                        crypto
                            .get(kind)
                            .ok_or_else(|| {
                                VeilidAPIError::invalid_argument(
                                    "unsupported cryptosystem",
                                    "kind",
                                    kind,
                                )
                            })
                            .map(|csv| self.add_crypto_system(csv)),
                    ),
                }
            }
            RequestOp::BestCryptoSystem => {
                let crypto = match self.api.crypto() {
                    Ok(v) => v,
                    Err(e) => {
                        return Response {
                            id,
                            op: ResponseOp::GetCryptoSystem {
                                result: to_json_api_result(Err(e)),
                            },
                        }
                    }
                };
                ResponseOp::BestCryptoSystem {
                    result: to_json_api_result(Ok(self.add_crypto_system(crypto.best()))),
                }
            }
            RequestOp::CryptoSystem(csr) => {
                let csv = match self.lookup_crypto_system(id, csr.cs_id) {
                    Ok(v) => v,
                    Err(e) => return e,
                };
                ResponseOp::CryptoSystem(self.process_crypto_system_request(csv, csr).await)
            }
            RequestOp::VerifySignatures {
                node_ids,
                data,
                signatures,
            } => {
                let crypto = match self.api.crypto() {
                    Ok(v) => v,
                    Err(e) => {
                        return Response {
                            id,
                            op: ResponseOp::GetCryptoSystem {
                                result: to_json_api_result(Err(e)),
                            },
                        }
                    }
                };
                ResponseOp::VerifySignatures {
                    result: to_json_api_result_with_vec_string(crypto.verify_signatures(
                        &node_ids,
                        &data,
                        &signatures,
                    )),
                }
            }
            RequestOp::GenerateSignatures { data, key_pairs } => {
                let crypto = match self.api.crypto() {
                    Ok(v) => v,
                    Err(e) => {
                        return Response {
                            id,
                            op: ResponseOp::GetCryptoSystem {
                                result: to_json_api_result(Err(e)),
                            },
                        }
                    }
                };
                ResponseOp::GenerateSignatures {
                    result: to_json_api_result_with_vec_string(crypto.generate_signatures(
                        &data,
                        &key_pairs,
                        |k, s| TypedSignature::new(k.kind, s),
                    )),
                }
            }
            RequestOp::GenerateKeyPair { kind } => ResponseOp::GenerateKeyPair {
                result: to_json_api_result_with_string(Crypto::generate_keypair(kind)),
            },
            RequestOp::Now => ResponseOp::Now {
                value: get_aligned_timestamp(),
            },
            RequestOp::Debug { command } => ResponseOp::Debug {
                result: to_json_api_result(self.api.debug(command).await),
            },
            RequestOp::VeilidVersionString => ResponseOp::VeilidVersionString {
                value: veilid_version_string(),
            },
            RequestOp::VeilidVersion => {
                let (major, minor, patch) = veilid_version();

                ResponseOp::VeilidVersion {
                    major,
                    minor,
                    patch,
                }
            }
        };

        Response { id, op }
    }
}
