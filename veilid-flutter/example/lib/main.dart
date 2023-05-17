import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter_acrylic/flutter_acrylic.dart';

import 'veilid_theme.dart';
import 'log.dart';
import 'app.dart';
import 'veilid_init.dart';

/////////////////////////////// Acrylic

bool get isDesktop {
  if (kIsWeb) return false;
  return [
    TargetPlatform.windows,
    TargetPlatform.linux,
    TargetPlatform.macOS,
  ].contains(defaultTargetPlatform);
}

Future<void> setupAcrylic() async {
  await Window.initialize();
  await Window.makeTitlebarTransparent();
  await Window.setEffect(
      effect: WindowEffect.aero, color: const Color(0xFFFFFFFF));
  await Window.setBlurViewState(MacOSBlurViewState.active);
}

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
