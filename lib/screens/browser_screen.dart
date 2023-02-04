import 'dart:async';

import 'package:flutter/material.dart';
import 'package:jenny/basic/methods.dart';
import 'package:jenny/screens/components/comic_pager.dart';
import 'package:jenny/screens/components/content_builder.dart';
import 'package:jenny/screens/components/floating_search_bar.dart';

import 'components/browser_bottom_sheet.dart';
import 'components/actions.dart';
import 'components/comic_floating_search_bar.dart';

class BrowserScreen extends StatefulWidget {
  final FloatingSearchBarController searchBarController;

  const BrowserScreen({Key? key, required this.searchBarController})
      : super(key: key);

  @override
  State<StatefulWidget> createState() => _BrowserScreenState();
}

class _BrowserScreenState extends State<BrowserScreen>
    with AutomaticKeepAliveClientMixin {
  @override
  bool get wantKeepAlive => true;

  late Future<CategoriesResponse> _future;
  late Key _key;
  String _slug = "";
  SortBy _sortBy = sortByDefault;

  Future<CategoriesResponse> _categories() async {
    final rsp = await methods.categories();
    blockStore = rsp.blocks;
    return rsp;
  }

  @override
  void initState() {
    _future = _categories();
    _key = UniqueKey();
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return Scaffold(
      appBar: AppBar(
        title: const Text("浏览"),
        actions: [
          IconButton(
            onPressed: () async {
              searchHistories = await methods.lastSearchHistories(20);
              widget.searchBarController.display(modifyInput: "");
            },
            icon: const Icon(Icons.search),
          ),
          const BrowserBottomSheetAction(),
        ],
      ),
      body: ContentBuilder(
        key: _key,
        future: _future,
        onRefresh: () async {
          setState(() {
            _future = _categories();
            _key = UniqueKey();
          });
        },
        successBuilder: (
          BuildContext context,
          AsyncSnapshot<CategoriesResponse> snapshot,
        ) {
          final categories = snapshot.requireData.categories;
          return Column(children: [
            SizedBox(
              height: 56,
              child: Container(
                padding: const EdgeInsets.only(top: 8),
                color: Theme.of(context).appBarTheme.backgroundColor,
                child: Row(
                  children: [
                    Expanded(
                      child: _MTabBar(
                        categories,
                        (index) {
                          setState(() {
                            _slug = categories[index].slug;
                          });
                        },
                      ),
                    ),
                    buildOrderSwitch(context, _sortBy, (value) {
                      setState(() {
                        _sortBy = value;
                      });
                    }),
                  ],
                ),
              ),
            ),
            Expanded(
              child: ComicPager(
                key: Key("$_slug:$_sortBy"),
                onPage: (int page) async {
                  final response = await methods.comics(_slug, _sortBy, page);
                  return InnerComicPage(
                    total: response.total,
                    list: response.content,
                  );
                },
              ),
            ),
          ]);
        },
      ),
    );
  }
}

class _MTabBar extends StatefulWidget {
  final List<Categories> categories;
  final void Function(int index) onTab;

  const _MTabBar(this.categories, this.onTab);

  @override
  State<StatefulWidget> createState() => _MTabBarState();
}

class _MTabBarState extends State<_MTabBar>
    with SingleTickerProviderStateMixin {
  late final TabController _tabController =
      TabController(length: widget.categories.length, vsync: this);

  @override
  void dispose() {
    _tabController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 50,
      child: TabBar(
          onTap: widget.onTab,
          controller: _tabController,
          isScrollable: true,
          indicatorSize: TabBarIndicatorSize.tab,
          padding: const EdgeInsets.only(left: 10, right: 10),
          indicator: BoxDecoration(
            color: Colors.grey.shade500.withOpacity(.3),
            borderRadius: const BorderRadius.only(
              topLeft: Radius.circular(5),
              topRight: Radius.circular(5),
            ),
          ),
          tabs: widget.categories
              .map((e) => Tab(
                    text: e.name,
                  ))
              .toList()),
    );
  }
}
