import 'package:flutter/material.dart';

import '../basic/commons.dart';
import '../basic/methods.dart';
import '../configs/is_pro.dart';
import 'components/right_click_pop.dart';

class ProScreen extends StatefulWidget {
  const ProScreen({Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() => _ProScreenState();
}

class _ProScreenState extends State<ProScreen> {
  String _username = "";

  @override
  void initState() {
    methods.loadUsername().then((value) {
      setState(() {
        _username = value;
      });
    });
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return rightClickPop(child: buildScreen(context), context: context);
  }

  Widget buildScreen(BuildContext context) {
    var size = MediaQuery.of(context).size;
    var min = size.width < size.height ? size.width : size.height;
    return Scaffold(
      appBar: AppBar(
        title: const Text("发电中心"),
      ),
      body: ListView(
        children: [
          SizedBox(
            width: min / 2,
            height: min / 2,
            child: Center(
              child: Icon(
                isPro ? Icons.offline_bolt : Icons.offline_bolt_outlined,
                size: min / 3,
                color: Colors.grey.shade500,
              ),
            ),
          ),
          Center(child: Text(_username)),
          Container(height: 20),
          const Divider(),
          const Padding(
            padding: EdgeInsets.all(20),
            child: Text(
              "登录账号才能确认发电状态\n"
              "点击\"我曾经发过电\"进同步发电状态\n"
              "点击\"我刚才发了电\"兑换作者给您的礼物卡\n"
              "去\"关于\"界面找到维护地址用爱发电",
            ),
          ),
          const Divider(),
          ListTile(
            title: const Text("发电详情"),
            subtitle: Text(
              isPro
                  ? "发电中 (${DateTime.fromMillisecondsSinceEpoch(1000 * isProEx).toString()})"
                  : "未发电",
            ),
          ),
          const Divider(),
          ListTile(
            title: const Text("我曾经发过电"),
            onTap: () async {
              try {
                await methods.reloadPro();
                defaultToast(context, "SUCCESS");
              } catch (e, s) {
                print("$e\n$s");
                defaultToast(context, "FAIL");
              }
              await reloadIsPro();
              setState(() {});
            },
          ),
          const Divider(),
          ListTile(
            title: const Text("我刚才发了电"),
            onTap: () async {
              final code = await displayTextInputDialog(context, title: "输入代码");
              if (code != null && code.isNotEmpty) {
                try {
                  await methods.inputCdKey(code);
                  defaultToast(context, "SUCCESS");
                } catch (e, s) {
                  print("$e\n$s");
                  defaultToast(context, "FAIL");
                }
              }
              await reloadIsPro();
              setState(() {});
            },
          ),
          const Divider(),
        ],
      ),
    );
  }
}
