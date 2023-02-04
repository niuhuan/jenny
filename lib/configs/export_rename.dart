import 'package:flutter/material.dart';

import '../basic/commons.dart';
import '../basic/methods.dart';
import 'is_pro.dart';

const _propertyName = "exportRename";
late bool _exportRename;

Future<void> initExportRename() async {
  _exportRename = (await methods.loadProperty(_propertyName)) == "true";
}

bool currentExportRename() {
  return _exportRename;
}

Future<void> _chooseExportRename(BuildContext context) async {
  String? result = await chooseListDialog<String>(context,
      title: "导出的时候重新命名", values: ["是", "否"]);
  if (result != null) {
    var target = result == "是";
    await methods.saveProperty(_propertyName, "$target");
    _exportRename = target;
  }
}

Widget exportRenameSetting() {
  return StatefulBuilder(
    builder: (BuildContext context, void Function(void Function()) setState) {
      return ListTile(
        title: Text(
          "导出的时候重新命名",
          style: TextStyle(
            color: !isPro ? Colors.grey : null,
          ),
        ),
        subtitle: Text(
          _exportRename ? "是" : "否",
          style: TextStyle(
            color: !isPro ? Colors.grey : null,
          ),
        ),
        onTap: () async {
          if (!isPro) {
            return;
          }
          await _chooseExportRename(context);
          setState(() {});
        },
      );
    },
  );
}
