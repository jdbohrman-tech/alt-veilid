import 'package:flutter/foundation.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:veilid/default_config.dart';

Future<void> testVeilidConfigDefaults() async {
  const programName = 'Veilid Tests';
  final defaultConfig =
      await getDefaultVeilidConfig(isWeb: kIsWeb, programName: programName);
  assert(defaultConfig.programName == programName, 'program name should match');
}
