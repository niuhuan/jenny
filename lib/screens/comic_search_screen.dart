import 'package:flutter/material.dart';
import 'package:jenny/basic/methods.dart';
import 'package:jenny/screens/components/floating_search_bar.dart';

import 'components/browser_bottom_sheet.dart';
import 'components/comic_floating_search_bar.dart';
import 'components/comic_pager.dart';
import 'components/actions.dart';
import 'components/right_click_pop.dart';

class ComicSearchScreen extends StatefulWidget {
  final String initKeywords;

  const ComicSearchScreen({required this.initKeywords, Key? key})
      : super(key: key);

  @override
  State<StatefulWidget> createState() => _ComicSearchScreenState();
}

class _ComicSearchScreenState extends State<ComicSearchScreen> {
  final _controller = FloatingSearchBarController();
  late var _keywords = widget.initKeywords;
  SortBy _sortBy = sortByDefault;

  @override
  Widget build(BuildContext context) {
    return rightClickPop(child: buildScreen(context), context: context);
  }

  Widget buildScreen(BuildContext context) {
    return ComicFloatingSearchBarScreen(
      controller: _controller,
      onQuery: (value) {
        setState(() {
          _keywords = value;
        });
      },
      child: Scaffold(
        appBar: AppBar(
          title: Text(_keywords),
          actions: [
            IconButton(
              onPressed: () async {
                searchHistories = await methods.lastSearchHistories(20);
                _controller.display(modifyInput: _keywords);
              },
              icon: const Icon(Icons.search),
            ),
            const BrowserBottomSheetAction(),
            buildOrderSwitch(context, _sortBy, (value) {
              setState(() {
                _sortBy = value;
              });
            }),
          ],
        ),
        body: ComicPager(
          key: Key("$_keywords:$_sortBy"),
          onPage: (int page) async {
            final response = await methods.comicSearch(
              _keywords,
              _sortBy,
              page,
            );
            return InnerComicPage(
              total: response.total,
              list: response.content,
              redirectAid: response.redirectAid,
            );
          },
        ),
      ),
    );
  }
}
