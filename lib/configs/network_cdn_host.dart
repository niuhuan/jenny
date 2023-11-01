import 'package:flutter/material.dart';
import 'package:jenny/basic/commons.dart';
import 'package:jenny/basic/methods.dart';

late String _cdnHost;

String _cdnHostName(String value) {
  return value == "0" ? "随机" : "分流$value";
}

String get currentCdnHostName => _cdnHostName(_cdnHost);

Future<void> initCdnHost() async {
  _cdnHost = await methods.loadCdnHost();
}

Future chooseCdnHost(BuildContext context) async {
  final choose = await chooseMapDialog(
    context,
    title: "图片分流",
    values: {
      "随机": "0",
      "分流1": "1",
      "分流2": "2",
      "分流3": "3",
      "分流4": "4",
      "分流5": "5",
    },
  );
  if (choose != null) {
    await methods.saveCdnHost(choose);
    _cdnHost = choose;
  }
}

Widget cdnHostSetting() {
  return StatefulBuilder(
    builder: (BuildContext context, void Function(void Function()) setState) {
      return ListTile(
        onTap: () async {
          await chooseCdnHost(context);
          setState(() {});
        },
        title: const Text("图片分流"),
        subtitle: Text(_cdnHostName(_cdnHost)),
      );
    },
  );
}
