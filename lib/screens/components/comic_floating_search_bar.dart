import 'package:event/event.dart';
import 'package:flutter/material.dart';
import 'package:jenny/basic/entities.dart';
import 'package:jenny/basic/methods.dart';

import '../../basic/commons.dart';
import 'floating_search_bar.dart';

final _event = Event();
final List<Block> _blockStore = [];
final List<SearchHistory> _histories = [];

set blockStore(List<Block> values) {
  _blockStore.clear();
  _blockStore.addAll(values);
  _event.broadcast();
}

set searchHistories(List<SearchHistory> values) {
  _histories.clear();
  _histories.addAll(values);
  _event.broadcast();
}

class ComicFloatingSearchBarScreen extends StatefulWidget {
  final FloatingSearchBarController controller;
  final Widget child;
  final ValueChanged<String>? onQuery;

  const ComicFloatingSearchBarScreen({
    Key? key,
    required this.controller,
    required this.child,
    this.onQuery,
  }) : super(key: key);

  @override
  State<StatefulWidget> createState() => _ComicFloatingSearchBarScreenState();
}

class _ComicFloatingSearchBarScreenState
    extends State<ComicFloatingSearchBarScreen> {
  final _panelController = ScrollController();

  @override
  void initState() {
    _event.subscribe(_setState);
    super.initState();
  }

  @override
  void dispose() {
    _event.unsubscribe(_setState);
    _panelController.dispose();
    super.dispose();
  }

  void _setState(_) {
    setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    return FloatingSearchBarScreen(
      controller: widget.controller,
      child: widget.child,
      onSubmitted: _onSubmitted,
      panel: _buildPanel(),
    );
  }

  void _onSubmitted(String value) {
    widget.controller.hide();
    if (value.isNotEmpty && widget.onQuery != null) {
      widget.onQuery!(value);
    }
  }

  Widget _buildPanel() {
    return ListView(
      controller: _panelController,
      padding: const EdgeInsets.all(10),
      children: [
        ..._buildHistory(),
        ..._buildTags(),
      ],
    );
  }

  List<Widget> _buildHistory() {
    if (_histories.isEmpty) {
      return [];
    }
    final List<Widget> widgets = [];
    widgets.add(_buildTitle("历史记录", clear: () async {
      String? choose = await chooseListDialog(
        context,
        values: ["是", "否"],
        title: "清除所有历史记录?",
      );
      if ("是" == choose) {
        await methods.clearAllSearchLog();
        _histories.clear();
        _setState(null);
      }
    }));
    widgets.add(Wrap(
      children: _histories.map((e) {
        return InkWell(
          onTap: () {
            _onSubmitted(e.searchQuery);
          },
          onLongPress: () async {
            String? choose = await chooseListDialog(
              context,
              values: ["是", "否"],
              title: "清除历史记录\"${e.searchQuery}\"?",
            );
            if ("是" == choose) {
              await methods.clearASearchLog(e.searchQuery);
              _histories.remove(e);
              _setState(null);
            }
          },
          child: Container(
            padding: const EdgeInsets.only(
              left: 10,
              right: 10,
              top: 3,
              bottom: 3,
            ),
            margin: const EdgeInsets.only(
              left: 5,
              right: 5,
              top: 3,
              bottom: 3,
            ),
            decoration: BoxDecoration(
              color: Colors.pink.shade100,
              border: Border.all(
                style: BorderStyle.solid,
                color: Colors.pink.shade400,
              ),
              borderRadius: const BorderRadius.all(Radius.circular(30)),
            ),
            child: Text(
              e.searchQuery,
              style: TextStyle(
                color: Colors.pink.shade500,
                height: 1.4,
              ),
              strutStyle: const StrutStyle(
                height: 1.4,
              ),
            ),
          ),
        );
      }).toList(),
    ));
    return widgets;
  }

  List<Widget> _buildTags() {
    final List<Widget> widgets = [];
    widgets.add(_buildTitle("板块"));
    for (final block in _blockStore) {
      widgets.add(_buildSubTitle(block.title));
      widgets.add(Wrap(
        children: block.content.map((e) {
          return InkWell(
            onTap: () {
              _onSubmitted(e);
            },
            child: Container(
              padding: const EdgeInsets.only(
                left: 10,
                right: 10,
                top: 3,
                bottom: 3,
              ),
              margin: const EdgeInsets.only(
                left: 5,
                right: 5,
                top: 3,
                bottom: 3,
              ),
              decoration: BoxDecoration(
                color: Colors.pink.shade100,
                border: Border.all(
                  style: BorderStyle.solid,
                  color: Colors.pink.shade400,
                ),
                borderRadius: const BorderRadius.all(Radius.circular(30)),
              ),
              child: Text(
                e,
                style: TextStyle(
                  color: Colors.pink.shade500,
                  height: 1.4,
                ),
                strutStyle: const StrutStyle(
                  height: 1.4,
                ),
              ),
            ),
          );
        }).toList(),
      ));
    }
    return widgets;
  }

  Widget _buildTitle(String title, {void Function()? clear}) {
    if (clear != null) {
      return Container(
        margin: const EdgeInsets.only(top: 10, bottom: 5),
        child: Row(
          children: [
            Text(
              title,
              style: const TextStyle(
                fontWeight: FontWeight.bold,
              ),
            ),
            Expanded(child: Container()),
            IconButton(
              onPressed: clear,
              icon: const Icon(Icons.close, size: 14, color: Colors.grey),
            ),
          ],
        ),
      );
    }
    return Container(
      margin: const EdgeInsets.only(top: 10, bottom: 5),
      child: Text(
        title,
        style: const TextStyle(
          fontWeight: FontWeight.bold,
        ),
      ),
    );
  }

  Widget _buildSubTitle(String title) {
    return Container(
      margin: const EdgeInsets.only(top: 4, bottom: 2),
      child: Text(
        title,
      ),
    );
  }
}
