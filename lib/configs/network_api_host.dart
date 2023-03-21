import 'package:flutter/material.dart';
import 'package:jenny/basic/commons.dart';
import 'package:jenny/basic/methods.dart';

late String _apiHost;

const apiHosts = {
  "原始": "0",
  "分流1": "1",
  "分流2": "2",
  "分流3": "3",
};

String _apiHostName(String value) {
  return value == "0" ? "原始" : "分流$value";
}

String get currentApiHostName => _apiHostName(_apiHost);

Future<void> initApiHost() async {
  _apiHost = await methods.loadApiHost();
}

Future<T?> chooseApiDialog<T>(BuildContext buildContext) async {
  return await showDialog<T>(
    context: buildContext,
    builder: (BuildContext context) {
      return SimpleDialog(
        title: const Text("API分流"),
        children: apiHosts.entries
            .map((e) => SimpleDialogOption(
                  child: ApiOptionRow(
                    e.key,
                    e.value,
                    key: Key("API:${e.value}"),
                  ),
                  onPressed: () {
                    Navigator.of(context).pop(e.value);
                  },
                ))
            .toList(),
      );
    },
  );
}

class ApiOptionRow extends StatefulWidget {
  final String title;
  final String value;

  const ApiOptionRow(this.title, this.value, {Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() => _ApiOptionRowState();
}

class _ApiOptionRowState extends State<ApiOptionRow> {
  late Future<int> _feature;

  @override
  void initState() {
    super.initState();
    _feature = methods.ping(widget.value);
  }

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Text(widget.title),
        Expanded(child: Container()),
        FutureBuilder(
          future: _feature,
          builder: (
            BuildContext context,
            AsyncSnapshot<int> snapshot,
          ) {
            if (snapshot.connectionState != ConnectionState.done) {
              return const PingStatus(
                "测速中",
                Colors.blue,
              );
            }
            if (snapshot.hasError) {
              return const PingStatus(
                "失败",
                Colors.red,
              );
            }
            int ping = snapshot.requireData;
            if (ping <= 200) {
              return PingStatus(
                "${ping}ms",
                Colors.green,
              );
            }
            if (ping <= 500) {
              return PingStatus(
                "${ping}ms",
                Colors.yellow,
              );
            }
            return PingStatus(
              "${ping}ms",
              Colors.orange,
            );
          },
        ),
      ],
    );
  }
}

class PingStatus extends StatelessWidget {
  final String title;
  final Color color;

  const PingStatus(this.title, this.color, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Text(
          '\u2022',
          style: TextStyle(
            color: color,
          ),
        ),
        Text(" $title"),
      ],
    );
  }
}

Future chooseApiHost(BuildContext context) async {
  final choose = await chooseApiDialog(context);
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
