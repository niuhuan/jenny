import 'package:event/event.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import '../basic/commons.dart';
import '../basic/methods.dart';

final _lightTheme = ThemeData.light().copyWith(
  useMaterial3: true,
  colorScheme: const ColorScheme(
    brightness: Brightness.light,
    primary: Color(0xFF0062A1),
    onPrimary: Color(0xFFFFFFFF),
    primaryContainer: Color(0xFFD0E4FF),
    onPrimaryContainer: Color(0xFF001D35),
    secondary: Color(0xFF984061),
    onSecondary: Color(0xFFFFFFFF),
    secondaryContainer: Color(0xFFFFD9E2),
    onSecondaryContainer: Color(0xFF3E001D),
    tertiary: Color(0xFF9A4523),
    onTertiary: Color(0xFFFFFFFF),
    tertiaryContainer: Color(0xFFFFDBCF),
    onTertiaryContainer: Color(0xFF380D00),
    error: Color(0xFFBA1A1A),
    errorContainer: Color(0xFFFFDAD6),
    onError: Color(0xFFFFFFFF),
    onErrorContainer: Color(0xFF410002),
    background: Color(0xFFFDFCFF),
    onBackground: Color(0xFF1A1C1E),
    surface: Color(0xFFFDFCFF),
    onSurface: Color(0xFF1A1C1E),
    surfaceVariant: Color(0xFFDFE3EB),
    onSurfaceVariant: Color(0xFF42474E),
    outline: Color(0xFF73777F),
    onInverseSurface: Color(0xFFF1F0F4),
    inverseSurface: Color(0xFF2F3033),
    inversePrimary: Color(0xFF9CCAFF),
    shadow: Color(0xFF000000),
    surfaceTint: Color(0xFF0062A1),
    outlineVariant: Color(0xFFC2C7CF),
    scrim: Color(0xFF000000),
  ),
);

final _darkTheme = ThemeData.dark().copyWith(
  useMaterial3: true,
  colorScheme: const ColorScheme(
    brightness: Brightness.dark,
    primary: Color(0xFFCFBCFF),
    onPrimary: Color(0xFF381E72),
    primaryContainer: Color(0xFF4F378A),
    onPrimaryContainer: Color(0xFFE9DDFF),
    secondary: Color(0xFFCBC2DB),
    onSecondary: Color(0xFF332D41),
    secondaryContainer: Color(0xFF4A4458),
    onSecondaryContainer: Color(0xFFE8DEF8),
    tertiary: Color(0xFFFFB873),
    onTertiary: Color(0xFF4B2800),
    tertiaryContainer: Color(0xFF6A3B00),
    onTertiaryContainer: Color(0xFFFFDCBF),
    error: Color(0xFFFFB4AB),
    errorContainer: Color(0xFF93000A),
    onError: Color(0xFF690005),
    onErrorContainer: Color(0xFFFFDAD6),
    background: Color(0xFF1C1B1E),
    onBackground: Color(0xFFE6E1E6),
    surface: Color(0xFF1C1B1E),
    onSurface: Color(0xFFE6E1E6),
    surfaceVariant: Color(0xFF49454E),
    onSurfaceVariant: Color(0xFFCAC4CF),
    outline: Color(0xFF948F99),
    onInverseSurface: Color(0xFF1C1B1E),
    inverseSurface: Color(0xFFE6E1E6),
    inversePrimary: Color(0xFF6750A4),
    shadow: Color(0xFF000000),
    surfaceTint: Color(0xFFCFBCFF),
    outlineVariant: Color(0xFF49454E),
    scrim: Color(0xFF000000),
  ),
);

ThemeData get lightTheme => theme != "2" ? _lightTheme : _darkTheme;

ThemeData get darkTheme => theme != "1" ? _darkTheme : _lightTheme;

const _propertyName = "theme";
late String theme = "0";

Map<String, String> _nameMap = {
  "0": "自动 (如果设备支持)",
  "1": "保持亮色",
  "2": "保持暗色",
};

Future initTheme() async {
  SystemChrome.setEnabledSystemUIMode(
    SystemUiMode.edgeToEdge,
    overlays: SystemUiOverlay.values,
  );
  theme = await methods.loadProperty(_propertyName);
  if (theme == "") {
    theme = "0";
  }
  themeEvent.broadcast();
  _reloadBarColor();
}

String themeName() {
  return _nameMap[theme] ?? "-";
}

Future chooseTheme(BuildContext context) async {
  String? choose = await chooseMapDialog(context,
      title: "选择主题",
      values: _nameMap.map((key, value) => MapEntry(value, key)));
  if (choose != null) {
    await methods.saveProperty(_propertyName, choose);
    theme = choose;
    themeEvent.broadcast();
    _reloadBarColor();
  }
}

reloadBarColor({bool op = false}) {
  _reloadBarColor(op: op);
}

_reloadBarColor({bool op = false}) {
  if (op) {
    switch (theme) {
      case '0':
        SystemChrome.setSystemUIOverlayStyle(const SystemUiOverlayStyle(
          systemStatusBarContrastEnforced: true,
          systemNavigationBarContrastEnforced: true,
        ));
        break;
      case '1':
        SystemChrome.setSystemUIOverlayStyle(const SystemUiOverlayStyle(
          systemNavigationBarIconBrightness: Brightness.dark,
          systemStatusBarContrastEnforced: true,
          systemNavigationBarContrastEnforced: true,
        ));
        break;
      case '2':
        SystemChrome.setSystemUIOverlayStyle(const SystemUiOverlayStyle(
          systemNavigationBarIconBrightness: Brightness.light,
          systemStatusBarContrastEnforced: true,
          systemNavigationBarContrastEnforced: true,
        ));
        break;
    }
  } else {
    switch (theme) {
      case '0':
        SystemChrome.setSystemUIOverlayStyle(const SystemUiOverlayStyle(
          systemStatusBarContrastEnforced: true,
          systemNavigationBarContrastEnforced: true,
        ));
        break;
      case '1':
        SystemChrome.setSystemUIOverlayStyle(const SystemUiOverlayStyle(
          systemNavigationBarColor: Colors.white,
          systemNavigationBarIconBrightness: Brightness.dark,
          systemStatusBarContrastEnforced: true,
          systemNavigationBarContrastEnforced: true,
        ));
        break;
      case '2':
        SystemChrome.setSystemUIOverlayStyle(const SystemUiOverlayStyle(
          systemNavigationBarColor: Colors.black87,
          systemNavigationBarIconBrightness: Brightness.light,
          systemStatusBarContrastEnforced: true,
          systemNavigationBarContrastEnforced: true,
        ));
        break;
    }
  }
}

final themeEvent = Event();

Widget themeSetting(BuildContext context) {
  return StatefulBuilder(
    builder: (BuildContext context, void Function(void Function()) setState) {
      return ListTile(
        onTap: () async {
          await chooseTheme(context);
          setState(() => {});
        },
        title: const Text("主题"),
        subtitle: Text(_nameMap[theme] ?? ""),
      );
    },
  );
}
