# Veilid Server
# =============
#
# Dev Node Configuration
#
# -----------------------------------------------------------

---
logging:
  system:
    enabled: true
    level: debug
  api:
    enabled: true
    level: debug
  terminal:
    enabled: false
core:
  capabilities:
    disable: ['APPM']
  network:
    upnp: false
    dht:
      min_peer_count: 10
    detect_address_changes: false
    routing_table:
      bootstrap: ['bootstrap.<your.domain>']
    network_key_password: '<your-chosen-passkey>'
  protected_store:
    insecure_fallback_directory: '/var/db/veilid-server/protected_store'
  table_store:
    directory: '/var/db/veilid-server/table_store'
  block_store:
    directory: '/var/db/veilid-server/block_store'