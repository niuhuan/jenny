/// 多线程下载并发数

import 'package:flutter/material.dart';
import 'package:jenny/basic/methods.dart';

import '../basic/commons.dart';
import 'is_pro.dart';

late int _downloadThreadCount;
int get downloadThreadCount => _downloadThreadCount;
const _values = [1, 2, 3, 4, 5];

Future initDownloadThreadCount() async {
  _downloadThreadCount = await methods.load_download_thread();
}

Widget downloadThreadCountSetting() {
  return StatefulBuilder(
    builder: (BuildContext context, void Function(void Function()) setState) {
      return ListTile(
        title: Text(
          "下载线程数" + (!isPro ? "(发电)" : ""),
          style: TextStyle(
            color: !isPro ? Colors.grey : null,
          ),
        ),
        subtitle: Text("$_downloadThreadCount"),
        onTap: () async {
          await chooseDownloadThread(context);
          setState(() {});
        },
      );
    },
  );
}

Future chooseDownloadThread(BuildContext context) async {
  if (!isPro) {
    defaultToast(context, "先发电才能使用多线程嗷");
    return;
  }
  int? value = await chooseListDialog(context,title: "选择下载线程数",  values:_values,);
  if (value != null) {
    await methods.set_download_thread(value);
    _downloadThreadCount = value;
  }
}
