import 'package:flutter/material.dart';
import 'package:jenny/basic/commons.dart';
import 'package:jenny/basic/entities.dart';

Widget buildOrderSwitch(
  BuildContext context,
  SortBy value,
  ValueChanged<SortBy> valueChanged,
) {
  final iconColor = Theme.of(context).appBarTheme.iconTheme?.color;
  return MaterialButton(
    onPressed: () async {
      final target = await chooseSortBy(context);
      if (target != null) {
        valueChanged(target);
      }
    },
    child: Column(
      children: [
        Expanded(child: Container()),
        Icon(
          Icons.sort,
          color: iconColor,
        ),
        Expanded(child: Container()),
        Text(value.toString(), style: TextStyle(color: iconColor)),
        Expanded(child: Container()),
      ],
    ),
  );
}
