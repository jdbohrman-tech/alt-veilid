use super::*;
use futures_util::FutureExt;

fn to_json_api_result<T: Clone + fmt::Debug + JsonSchema>(
    r: VeilidAPIResult<T>,
) -> json_api::ApiResult<T> {
    match r {
        Err(e) => json_api::ApiResult::Err { error: e },
        Ok(v) => json_api::ApiResult::Ok { value: v },
    }
}

fn to_json_api_result_with_string<T: Clone + fmt::Debug>(
    r: VeilidAPIResult<T>,
) -> json_api::ApiResultWithString<T> {
    match r {
        Err(e) => json_api::ApiResultWithString::Err { error: e },
        Ok(v) => json_api::ApiResultWithString::Ok { value: v },
    }
}

fn to_json_api_result_with_vec_u8(r: VeilidAPIResult<Vec<u8>>) -> json_api::ApiResultWithVecU8 {
    match r {
        Err(e) => json_api::ApiResultWithVecU8::Err { error: e },
        Ok(v) => json_api::ApiResultWithVecU8::Ok { value: v },
    }
}

fn to_json_api_result_with_vec_vec_u8(
    r: VeilidAPIResult<Vec<Vec<u8>>>,
) -> json_api::ApiResultWithVecVecU8 {
    match r {
        Err(e) => json_api::ApiResultWithVecVecU8::Err { error: e },
        Ok(v) => json_api::ApiResultWithVecVecU8::Ok {
            value: v.into_iter().map(|v| VecU8 { value: v }).collect(),
        },
    }
}

pub struct JsonRequestProcessor {
    api: VeilidAPI,
    routing_contexts: Mutex<BTreeMap<u32, RoutingContext>>,
    table_dbs: Mutex<BTreeMap<u32, TableDB>>,
    table_db_transactions: Mutex<BTreeMap<u32, TableDBTransaction>>,
}

impl JsonRequestProcessor {
    pub fn new(api: VeilidAPI) -> Self {
        Self {
            api,
            routing_contexts: Default::default(),
            table_dbs: Default::default(),
            table_db_transactions: Default::default(),
        }
    }

    // Routing Context
    fn add_routing_context(&self, routing_context: RoutingContext) -> u32 {
        let mut next_id: u32 = 1;
        let mut rc = self.routing_contexts.lock();
        while rc.contains_key(&next_id) {
            next_id += 1;
        }
        rc.insert(next_id, routing_context);
        next_id
    }
    fn lookup_routing_context(&self, id: u32, rc_id: u32) -> Result<RoutingContext, Response> {
        let routing_contexts = self.routing_contexts.lock();
        let Some(routing_context) = routing_contexts.get(&rc_id).cloned() else {
            return Err(Response {
                id,
                op: ResponseOp::RoutingContext(RoutingContextResponse {
                    rc_id,
                    rc_op: RoutingContextResponseOp::InvalidId
                })
            });
        };
        Ok(routing_context)
    }
    fn release_routing_context(&self, id: u32) -> i32 {
        let mut rc = self.routing_contexts.lock();
        if rc.remove(&id).is_none() {
            return 0;
        }
        return 1;
    }

    // TableDB
    fn add_table_db(&self, table_db: TableDB) -> u32 {
        let mut next_id: u32 = 1;
        let mut rc = self.table_dbs.lock();
        while rc.contains_key(&next_id) {
            next_id += 1;
        }
        rc.insert(next_id, table_db);
        next_id
    }
    fn lookup_table_db(&self, id: u32, db_id: u32) -> Result<TableDB, Response> {
        let table_dbs = self.table_dbs.lock();
        let Some(table_db) = table_dbs.get(&db_id).cloned() else {
            return Err(Response {
                id,
                op: ResponseOp::TableDb(TableDbResponse {
                    db_id,
                    db_op: TableDbResponseOp::InvalidId
                })
            });
        };
        Ok(table_db)
    }
    fn release_table_db(&self, id: u32) -> i32 {
        let mut rc = self.table_dbs.lock();
        if rc.remove(&id).is_none() {
            return 0;
        }
        return 1;
    }

    // TableDBTransaction
    fn add_table_db_transaction(&self, tdbt: TableDBTransaction) -> u32 {
        let mut next_id: u32 = 1;
        let mut tdbts = self.table_db_transactions.lock();
        while tdbts.contains_key(&next_id) {
            next_id += 1;
        }
        tdbts.insert(next_id, tdbt);
        next_id
    }
    fn lookup_table_db_transaction(
        &self,
        id: u32,
        tx_id: u32,
    ) -> Result<TableDBTransaction, Response> {
        let table_db_transactions = self.table_db_transactions.lock();
        let Some(table_db_transaction) = table_db_transactions.get(&tx_id).cloned() else {
            return Err(Response {
                id,
                op: ResponseOp::TableDbTransaction(TableDbTransactionResponse {
                    tx_id,
                    tx_op: TableDbTransactionResponseOp::InvalidId
                })
            });
        };
        Ok(table_db_transaction)
    }
    fn release_table_db_transaction(&self, id: u32) -> i32 {
        let mut tdbts = self.table_db_transactions.lock();
        if tdbts.remove(&id).is_none() {
            return 0;
        }
        return 1;
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

        Err(VeilidAPIError::invalid_target())
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
            RoutingContextRequestOp::WithPrivacy => RoutingContextResponseOp::WithPrivacy {
                result: to_json_api_result(
                    routing_context
                        .clone()
                        .with_privacy()
                        .map(|new_rc| self.add_routing_context(new_rc)),
                ),
            },
            RoutingContextRequestOp::WithCustomPrivacy { stability } => {
                RoutingContextResponseOp::WithCustomPrivacy {
                    result: to_json_api_result(
                        routing_context
                            .clone()
                            .with_custom_privacy(stability)
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
            RoutingContextRequestOp::AppCall { target, request } => {
                RoutingContextResponseOp::AppCall {
                    result: to_json_api_result_with_vec_u8(
                        self.parse_target(target)
                            .then(|tr| async { routing_context.app_call(tr?, request).await })
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
            RoutingContextRequestOp::CreateDhtRecord { kind, schema } => {
                RoutingContextResponseOp::CreateDhtRecord {
                    result: to_json_api_result(
                        routing_context.create_dht_record(kind, schema).await,
                    ),
                }
            }
            RoutingContextRequestOp::OpenDhtRecord { key, writer } => {
                RoutingContextResponseOp::OpenDhtRecord {
                    result: to_json_api_result(routing_context.open_dht_record(key, writer).await),
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
                table_db_transaction.store(col, &key, &value);
                TableDbTransactionResponseOp::Store {}
            }
            TableDbTransactionRequestOp::Delete { col, key } => {
                table_db_transaction.delete(col, &key);
                TableDbTransactionResponseOp::Delete {}
            }
        };
        TableDbTransactionResponse {
            tx_id: tdtr.tx_id,
            tx_op,
        }
    }

    pub async fn process_request(&self, request: Request) -> Response {
        let id = request.id;

        let op = match request.op {
            RequestOp::GetState => ResponseOp::GetState {
                result: to_json_api_result(self.api.get_state().await),
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
                value: self.add_routing_context(self.api.routing_context()),
            },
            RequestOp::RoutingContext(rcr) => {
                let routing_context = match self.lookup_routing_context(id, rcr.rc_id) {
                    Ok(v) => v,
                    Err(e) => return e,
                };
                ResponseOp::RoutingContext(
                    self.process_routing_context_request(routing_context, rcr)
                        .await,
                )
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
            RequestOp::GetCryptoSystem { kind } => todo!(),
            RequestOp::BestCryptoSystem => todo!(),
            RequestOp::CryptoSystem(_) => todo!(),
            RequestOp::VerifySignatures {
                node_ids,
                data,
                signatures,
            } => todo!(),
            RequestOp::GenerateSignatures { data, key_pairs } => todo!(),
            RequestOp::GenerateKeyPair { kind } => todo!(),
            RequestOp::Now => todo!(),
            RequestOp::Debug { command } => todo!(),
            RequestOp::VeilidVersionString => todo!(),
            RequestOp::VeilidVersion => todo!(),
        };

        Response { id, op }
    }
}
