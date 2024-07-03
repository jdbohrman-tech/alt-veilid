import 'package:flutter/foundation.dart';
import 'package:veilid/veilid.dart';

// Initialize Veilid
// Call only once.
void veilidInit() {
  if (kIsWeb) {
    const platformConfig = VeilidWASMConfig(
        logging: VeilidWASMConfigLogging(
            performance: VeilidWASMConfigLoggingPerformance(
                enabled: true,
                level: VeilidConfigLogLevel.debug,
                logsInTimings: true,
                logsInConsole: true),
            api: VeilidWASMConfigLoggingApi(
                enabled: true, level: VeilidConfigLogLevel.info)));
    Veilid.instance.initializeVeilidCore(platformConfig.toJson());
  } else {
    const platformConfig = VeilidFFIConfig(
        logging: VeilidFFIConfigLogging(
            terminal: VeilidFFIConfigLoggingTerminal(
              enabled: false,
              level: VeilidConfigLogLevel.debug,
            ),
            otlp: VeilidFFIConfigLoggingOtlp(
                enabled: false,
                level: VeilidConfigLogLevel.trace,
                grpcEndpoint: 'localhost:4317',
                serviceName: 'VeilidExample'),
            api: VeilidFFIConfigLoggingApi(
                enabled: true, level: VeilidConfigLogLevel.info),
            flame: VeilidFFIConfigLoggingFlame(enabled: false, path: '')));
    Veilid.instance.initializeVeilidCore(platformConfig.toJson());
  }
}
