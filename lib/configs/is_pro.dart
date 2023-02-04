import 'package:event/event.dart';
import '../basic/methods.dart';

var isPro = false;
var isProEx = 0;

final proEvent = Event();

Future reloadIsPro() async {
  final p = await methods.isPro();
  isPro = p.isPro;
  isProEx = p.expire;
  proEvent.broadcast();
}
