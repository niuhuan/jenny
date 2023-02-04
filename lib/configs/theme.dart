import 'package:event/event.dart';
import 'package:flutter/material.dart';

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
    primary: Color(0xFFD0BCFF),
    onPrimary: Color(0xFF391E72),
    primaryContainer: Color(0xFF50378A),
    onPrimaryContainer: Color(0xFFE9DDFF),
    secondary: Color(0xFF63DBB5),
    onSecondary: Color(0xFF00382A),
    secondaryContainer: Color(0xFF00513E),
    onSecondaryContainer: Color(0xFF81F8D0),
    tertiary: Color(0xFF80D0FF),
    onTertiary: Color(0xFF00344B),
    tertiaryContainer: Color(0xFF004C6A),
    onTertiaryContainer: Color(0xFFC5E7FF),
    error: Color(0xFFFFB4AB),
    errorContainer: Color(0xFF93000A),
    onError: Color(0xFF690005),
    onErrorContainer: Color(0xFFFFDAD6),
    background: Color(0xFF001F2A),
    onBackground: Color(0xFFBFE9FF),
    surface: Color(0xFF001F2A),
    onSurface: Color(0xFFBFE9FF),
    surfaceVariant: Color(0xFF49454E),
    onSurfaceVariant: Color(0xFFCAC4CF),
    outline: Color(0xFF948F99),
    onInverseSurface: Color(0xFF001F2A),
    inverseSurface: Color(0xFFBFE9FF),
    inversePrimary: Color(0xFF684FA4),
    shadow: Color(0xFF000000),
    surfaceTint: Color(0xFFD0BCFF),
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
  theme = await methods.loadProperty(_propertyName);
  if (theme == "") {
    theme = "0";
  }
}

String themeName() {
  return _nameMap[theme] ?? "-";
}

Future chooseTheme(BuildContext context) async {
  String? choose = await chooseMapDialog(context,
      title: "选择自动清理时间",
      values: _nameMap.map((key, value) => MapEntry(value, key)));
  if (choose != null) {
    await methods.saveProperty(_propertyName, choose);
    theme = choose;
    themeEvent.broadcast();
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
