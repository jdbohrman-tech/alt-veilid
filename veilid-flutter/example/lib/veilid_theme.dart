// Veilid Colors
// -------------
//
// Base is origin color as annotated
//
// Shades from material color tool at:
// https://m2.material.io/design/color/the-color-system.html#tools-for-picking-colors
//

import 'package:flutter/material.dart';

/////////////////////////////////////////////////////////
// Colors

const Map<int, Color> primaryColorSwatch = {
  50: Color(0xffe9e9f3),
  100: Color(0xffc7c8e2),
  200: Color(0xffa2a5ce),
  300: Color(0xff7f82ba),
  400: Color(0xff6667ab), // Base (6667ab)
  500: Color(0xff4f4d9d),
  600: Color(0xff484594),
  700: Color(0xff403b88),
  800: Color(0xff39327c),
  900: Color(0xff2b2068),
};

const MaterialColor materialPrimaryColor =
    MaterialColor(0xff6667ab, primaryColorSwatch);

const Map<int, Color> primaryComplementaryColorSwatch = {
  50: Color(0xfffafdee),
  100: Color(0xfff4f9d3),
  200: Color(0xffedf6b8),
  300: Color(0xffe7f29f),
  400: Color(0xffe2ed8d),
  500: Color(0xffdde97d),
  600: Color(0xffd0d776),
  700: Color(0xffbdc16d),
  800: Color(0xffabaa66), // Base (#abaa66)
  900: Color(0xff8b845c),
};

const MaterialColor materialPrimaryComplementaryColor =
    MaterialColor(0xffabaa66, primaryComplementaryColorSwatch);

const Map<int, Color> primaryTriadicColorASwatch = {
  50: Color(0xfff0e4f0),
  100: Color(0xffdabcdb),
  200: Color(0xffc290c3),
  300: Color(0xffaa66ab), // Base (#aa66ab)
  400: Color(0xff98489a),
  500: Color(0xff892a8c),
  600: Color(0xff7d2786),
  700: Color(0xff6d217e),
  800: Color(0xff5e1b76),
  900: Color(0xff441168),
};

const MaterialColor materialPrimaryTriadicColorA =
    MaterialColor(0xffaa66ab, primaryTriadicColorASwatch);

const Map<int, Color> primaryTriadicColorBSwatch = {
  50: Color(0xffffe3dc),
  100: Color(0xfff7c4c2),
  200: Color(0xffdba2a2),
  300: Color(0xffc08180),
  400: Color(0xffab6667), // Base (#ab6667)
  500: Color(0xff964c4f),
  600: Color(0xff894347),
  700: Color(0xff78373d),
  800: Color(0xff672b35),
  900: Color(0xff551e2a),
};

const MaterialColor materialPrimaryTriadicColorB =
    MaterialColor(0xffab6667, primaryTriadicColorBSwatch);

const Map<int, Color> secondaryColorSwatch = {
  50: Color(0xffe3e8f7),
  100: Color(0xffb8c6eb),
  200: Color(0xff87a1dd), // Base (#87a1dd)
  300: Color(0xff527dce),
  400: Color(0xff1a61c1),
  500: Color(0xff0048b5),
  600: Color(0xff0040ab),
  700: Color(0xff0037a0),
  800: Color(0xff002e94),
  900: Color(0xff001d7f),
};

const MaterialColor materialSecondaryColor =
    MaterialColor(0xff87a1dd, secondaryColorSwatch);

const Map<int, Color> secondaryComplementaryColorSwatch = {
  50: Color(0xfff6f1e2),
  100: Color(0xffeadbb6),
  200: Color(0xffddc387), // Base (#ddc387)
  300: Color(0xffd2ac55),
  400: Color(0xffcd9c2d),
  500: Color(0xffc88c05),
  600: Color(0xffc58200),
  700: Color(0xffbf7400),
  800: Color(0xffb96700),
  900: Color(0xffb15000),
};

const MaterialColor materialSecondaryComplementaryColor =
    MaterialColor(0xffddc387, secondaryComplementaryColorSwatch);

const Map<int, Color> backgroundColorSwatch = {
  50: Color(0xffe3e5eb),
  100: Color(0xffb9bdce),
  200: Color(0xff8c93ac),
  300: Color(0xff626a8c),
  400: Color(0xff454d76),
  500: Color(0xff273263),
  600: Color(0xff222c5b),
  700: Color(0xff1a2451),
  800: Color(0xff131c45),
  900: Color(0xff0b0b2f), // Base (#0b0b2f)
};

const MaterialColor materialBackgroundColor =
    MaterialColor(0xff0b0b2f, backgroundColorSwatch);

const Map<int, Color> backgroundComplementaryColorSwatch = {
  50: Color(0xfffffed2),
  100: Color(0xfffdf9cd),
  200: Color(0xfff8f5c8),
  300: Color(0xfff3efc3),
  400: Color(0xffd1cea3),
  500: Color(0xffb4b187),
  600: Color(0xff89865e),
  700: Color(0xff73714a),
  800: Color(0xff53512c),
  900: Color(0xff2f2f0b), // Base (#2f2f0b)
};

const MaterialColor materialBackgroundComplementaryColor =
    MaterialColor(0xff2f2f0b, backgroundComplementaryColorSwatch);

const Map<int, Color> desaturatedColorSwatch = {
  50: Color(0xfff7fbff),
  100: Color(0xfff2f6ff),
  200: Color(0xffedf1fd),
  300: Color(0xffe3e7f2),
  400: Color(0xffc1c5d0), // Base (#c1c5d0)
  500: Color(0xffa3a7b2),
  600: Color(0xff797d87),
  700: Color(0xff656973),
  800: Color(0xff464952),
  900: Color(0xff242830),
};

const MaterialColor materialDesaturatedColor =
    MaterialColor(0xffc1c5d0, desaturatedColorSwatch);

const Map<int, Color> desaturatedComplementaryColorSwatch = {
  50: Color(0xffecebe5),
  100: Color(0xffd0ccc1), // Base (#d0ccc1)
  200: Color(0xffb0aa9a),
  300: Color(0xff908972),
  400: Color(0xff796f54),
  500: Color(0xff615837),
  600: Color(0xff584e31),
  700: Color(0xff4a4128),
  800: Color(0xff3e341f),
  900: Color(0xff312715),
};

const MaterialColor materialDesaturatedComplementaryColor =
    MaterialColor(0xffd0ccc1, desaturatedComplementaryColorSwatch);

const Map<int, Color> auxiliaryColorSwatch = {
  50: Color(0xffe7e4da), // Base (#e7e4da)
  100: Color(0xffc2bbac),
  200: Color(0xff988e7b),
  300: Color(0xff6f634c),
  400: Color(0xff53472b),
  500: Color(0xff372c0a),
  600: Color(0xff302403),
  700: Color(0xff261a00),
  800: Color(0xff1e0c00),
  900: Color(0xff160000),
};

const MaterialColor materialAuxiliaryColor =
    MaterialColor(0xffe7e4da, auxiliaryColorSwatch);

const Map<int, Color> auxiliaryComplementaryColorSwatch = {
  50: Color(0xffdadde7), // Base (#dadde7)
  100: Color(0xffa2abc6),
  200: Color(0xff6575a2),
  300: Color(0xff224580),
  400: Color(0xff00266c),
  500: Color(0xff000357),
  600: Color(0xff000051),
  700: Color(0xff000051),
  800: Color(0xff000050),
  900: Color(0xff00004f),
};

const MaterialColor materialAuxiliaryComplementaryColor =
    MaterialColor(0xffdadde7, auxiliaryComplementaryColorSwatch);

const Map<int, Color> popColorSwatch = {
  50: Color(0xfffee5f5),
  100: Color(0xfffbbde7),
  200: Color(0xfff88fd9),
  300: Color(0xfff259c9), // Base (#f259c9)
  400: Color(0xffec15bd),
  500: Color(0xffe100b0),
  600: Color(0xffd200ac),
  700: Color(0xffbe00a7),
  800: Color(0xffad00a1),
  900: Color(0xff8e0097),
};

const MaterialColor materialPopColor =
    MaterialColor(0xfff259c9, popColorSwatch);

const Map<int, Color> popComplentaryColorSwatch = {
  50: Color(0xffe6fdea),
  100: Color(0xffc2f9cb),
  200: Color(0xff96f6a9),
  300: Color(0xff59f282), // Base (#59f282)
  400: Color(0xff00ec60),
  500: Color(0xff00e446),
  600: Color(0xff00d33b),
  700: Color(0xff00bf2d),
  800: Color(0xff00ad21),
  900: Color(0xff008b05),
};

const MaterialColor materialPopComplementaryColor =
    MaterialColor(0xff59f282, popComplentaryColorSwatch);

/////////////////////////////////////////////////////////
// Spacing

const kDefaultSpacingFactor = 4.0;

const kDefaultMonoTerminalFontFamily = "Fira Code";
const kDefaultMonoTerminalFontHeight = 1.2;
const kDefaultMonoTerminalFontSize = 12.0;

double spacingFactor(double multiplier) {
  return multiplier * kDefaultSpacingFactor;
}

Padding pad(Widget child) {
  return Padding(
      padding: const EdgeInsets.all(kDefaultSpacingFactor), child: child);
}

/////////////////////////////////////////////////////////
// Theme

InputDecoration newInputDecoration(String labelText, bool enabled) {
  return InputDecoration(
      labelText: labelText,
      fillColor: enabled
          ? materialPrimaryColor.shade200
          : materialPrimaryColor.shade200.withOpacity(0.5));
}

InputDecorationTheme newInputDecorationTheme() {
  return InputDecorationTheme(
      border: const OutlineInputBorder(),
      filled: true,
      fillColor: materialPrimaryColor.shade200,
      disabledBorder: const OutlineInputBorder(
          borderSide:
              BorderSide(color: Color.fromARGB(0, 0, 0, 0), width: 0.0)),
      focusedBorder: OutlineInputBorder(
          borderSide:
              BorderSide(color: materialPrimaryColor.shade900, width: 0.0)),
      floatingLabelBehavior: FloatingLabelBehavior.never,
      floatingLabelStyle: TextStyle(
        color: materialPrimaryColor.shade900,
        letterSpacing: 1.2,
      ));
}

ThemeData newVeilidTheme() {
  return ThemeData(
    primarySwatch: materialPrimaryColor,
    secondaryHeaderColor: materialSecondaryColor,
    visualDensity: VisualDensity.adaptivePlatformDensity,
    inputDecorationTheme: newInputDecorationTheme(),
  );
}
