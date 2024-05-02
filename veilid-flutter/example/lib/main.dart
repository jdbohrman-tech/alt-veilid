import 'package:flutter/material.dart';

import 'app.dart';
import 'log.dart';
import 'veilid_init.dart';
import 'veilid_theme.dart';

/////////////////////////////// Entrypoint
void main() {
  WidgetsFlutterBinding.ensureInitialized();

  // Initialize Log
  initLoggy();

  // Initialize Veilid
  veilidInit();

  // Run the app
  runApp(MaterialApp(
      title: 'Veilid Plugin Demo',
      theme: newVeilidTheme(),
      home: const MyApp()));
}
