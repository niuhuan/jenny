import 'package:flutter/material.dart';
import 'package:jenny/screens/components/mouse_and_touch_scroll_behavior.dart';
import 'package:jenny/screens/init_screen.dart';
import 'basic/desktop.dart';
import 'basic/navigator.dart';
import 'configs/theme.dart';

void main() async {
  runApp(const Jenny());
}

class Jenny extends StatefulWidget {
  const Jenny({Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() => _JennyState();
}

class _JennyState extends State<Jenny> {

  @override
  void initState() {
    onDesktopStart();
    themeEvent.subscribe(_setState);
    super.initState();
  }

  @override
  void dispose() {
    onDesktopStop();
    themeEvent.unsubscribe(_setState);
    super.dispose();
  }

  _setState(_) {
    setState(() => {});
  }
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      scrollBehavior: mouseAndTouchScrollBehavior,
      debugShowCheckedModeBanner: false,
      theme: lightTheme,
      darkTheme: darkTheme,
      navigatorObservers: [routeObserver],
      home: const InitScreen(),
    );
  }
}
