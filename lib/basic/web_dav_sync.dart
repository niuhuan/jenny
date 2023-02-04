import 'package:flutter/material.dart';
import 'package:jenny/basic/methods.dart';
import 'package:jenny/configs/web_dav_url.dart';

import '../configs/is_pro.dart';
import '../configs/web_dav_password.dart';
import '../configs/web_dav_sync_switch.dart';
import '../configs/web_dav_username.dart';
import 'commons.dart';

Future webDavSync(BuildContext context) async {
  try {
    await methods.webDavSync({
      "url": currentWebDavUrl,
      "username": currentWebUserName,
      "password": currentWebDavPassword,
      "direction": "Merge",
    });
    defaultToast(context, "WebDav 同步成功");
  } catch (e, s) {
    print("$e\n$s");
    defaultToast(context, "WebDav 同步失败 : $e");
  }
}

Future webDavSyncAuto(BuildContext context) async {
  if (currentWebDavSyncSwitch() && isPro) {
    await webDavSync(context);
  }
}

var syncing = false;

Widget webDavSyncClick(BuildContext context) {
  return ListTile(
    title: const Text("立即同步"),
    onTap: () async {
      if (syncing) return;
      syncing = true;
      try {
        await webDavSync(context);
      } finally {
        syncing = false;
      }
    },
  );
}
