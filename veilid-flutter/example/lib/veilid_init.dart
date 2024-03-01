import 'package:flutter/foundation.dart';
import 'package:veilid/veilid.dart';

// Initialize Veilid
// Call only once.
void veilidInit() {
  if (kIsWeb) {
    var platformConfig = const VeilidWASMConfig(
        logging: VeilidWASMConfigLogging(
            performance: VeilidWASMConfigLoggingPerformance(
                enabled: true,
                level: VeilidConfigLogLevel.debug,
                logsInTimings: true,
                logsInConsole: false,
                ignoreLogTargets: []),
            api: VeilidWASMConfigLoggingApi(
                enabled: true,
                level: VeilidConfigLogLevel.info,
                ignoreLogTargets: [])));
    Veilid.instance.initializeVeilidCore(platformConfig.toJson());
  } else {
    var platformConfig = const VeilidFFIConfig(
        logging: VeilidFFIConfigLogging(
            terminal: VeilidFFIConfigLoggingTerminal(
                enabled: false,
                level: VeilidConfigLogLevel.debug,
                ignoreLogTargets: []),
            otlp: VeilidFFIConfigLoggingOtlp(
                enabled: false,
                level: VeilidConfigLogLevel.trace,
                grpcEndpoint: "localhost:4317",
                serviceName: "VeilidExample",
                ignoreLogTargets: []),
            api: VeilidFFIConfigLoggingApi(
                enabled: true,
                level: VeilidConfigLogLevel.info,
                ignoreLogTargets: [])));
    Veilid.instance.initializeVeilidCore(platformConfig.toJson());
  }
}
