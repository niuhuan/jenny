import 'dart:io';

import 'package:flutter/material.dart';

Widget importNotice(BuildContext context) {
  if (Platform.isAndroid) {
    return Container(
      margin: const EdgeInsets.all(15),
      padding: const EdgeInsets.all(15),
      color: (Theme
          .of(context)
          .textTheme
          .bodyText1
          ?.color ?? Colors.black)
          .withOpacity(.01),
      child: const Text(
        "您正在使用安卓设备:\n如果不能导入导出并且提示权限不足, 可以尝试在Download或Document下建立子目录进行导入",
      ),
    );
  }
  return Container();
}
