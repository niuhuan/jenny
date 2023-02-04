import 'package:flutter/material.dart';
import 'package:jenny/basic/commons.dart';
import 'package:jenny/basic/methods.dart';

late String _apiHost;

String _apiHostName(String value) {
  return value == "0" ? "原始" : "分流$value";
}

String get currentApiHostName => _apiHostName(_apiHost);

Future<void> initApiHost() async {
  _apiHost = await methods.loadApiHost();
}

Future chooseApiHost(BuildContext context) async {
  final choose = await chooseMapDialog(
    context,
    title: "API分流",
    values: {
      "原始": "0",
      "分流1": "1",
      "分流2": "2",
      "分流3": "3",
    },
  );
  if (choose != null) {
    await methods.saveApiHost(choose);
    _apiHost = choose;
  }
}

Widget apiHostSetting() {
  return StatefulBuilder(
    builder: (BuildContext context, void Function(void Function()) setState) {
      return ListTile(
        onTap: () async {
          await chooseApiHost(context);
          setState(() {});
        },
        title: const Text("API分流"),
        subtitle: Text(_apiHostName(_apiHost)),
      );
    },
  );
}
