import 'dart:io';

import 'package:jenny/basic/methods.dart';
import 'package:window_manager/window_manager.dart';

onDesktopStart() {
  if (Platform.isWindows || Platform.isMacOS || Platform.isLinux) {
    windowManager.ensureInitialized();
    windowManager.addListener(winListener);
  }
}

onDesktopStop() {
  if (Platform.isWindows || Platform.isMacOS || Platform.isLinux) {
    windowManager.removeListener(winListener);
  }
}

const winListener = WinListener();

class WinListener with WindowListener {
  const WinListener();

  @override
  void onWindowResize() async {
    saveSize();
  }

  saveSize() async {
    final size = await windowManager.getSize();
    final windowWidth = size.width.toInt();
    final windowHeight = size.height.toInt();
    await methods.saveProperty("window_width", "$windowWidth");
    await methods.saveProperty("window_height", "$windowHeight");
  }
}
