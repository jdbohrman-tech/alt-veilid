import 'package:flutter_test/flutter_test.dart';
import 'package:veilid/value_subkey_range.dart';

void expectCommute<T>(T? Function(T, T) func, T a, T b, T? c) {
  expect(func(a, b), c);
  expect(func(b, a), c);
}

ValueSubkeyRange? vsrUnion(ValueSubkeyRange a, ValueSubkeyRange b) =>
    a.union(b);

ValueSubkeyRange? vsrIntersect(ValueSubkeyRange a, ValueSubkeyRange b) =>
    a.intersect(b);

List<ValueSubkeyRange> lvsrUnion(
        List<ValueSubkeyRange> a, List<ValueSubkeyRange> b) =>
    a.unionSubkeys(b);

List<ValueSubkeyRange> lvsrIntersect(
        List<ValueSubkeyRange> a, List<ValueSubkeyRange> b) =>
    a.intersectSubkeys(b);

Future<void> testValueSubkeyRange() async {
  final a = ValueSubkeyRange.single(1);
  const b = ValueSubkeyRange(low: 2, high: 3);
  const c = ValueSubkeyRange(low: 3, high: 4);

  expectCommute(vsrUnion, a, a, a);
  expectCommute(vsrUnion, a, b, const ValueSubkeyRange(low: 1, high: 3));
  expectCommute<ValueSubkeyRange>(vsrUnion, a, c, null);
  expectCommute(vsrUnion, b, c, const ValueSubkeyRange(low: 2, high: 4));

  const d = ValueSubkeyRange(low: 0, high: 3);

  expectCommute<ValueSubkeyRange>(vsrIntersect, a, b, null);
  expectCommute(vsrIntersect, c, d, ValueSubkeyRange.single(3));
  expectCommute(vsrIntersect, d, d, d);
}

Future<void> testValueSubkeyRangeList() async {
  final a = [ValueSubkeyRange.single(1)];
  final b = [ValueSubkeyRange.fromIntPair((2, 3))];
  final c = [ValueSubkeyRange.make(3, 4)];

  expectCommute(lvsrUnion, a, a, a);
  expectCommute(lvsrUnion, a, b, [const ValueSubkeyRange(low: 1, high: 3)]);
  expectCommute<List<ValueSubkeyRange>>(
      lvsrUnion, a, c, ListValueSubkeyRangeExt.fromIntPairs([(1, 1), (3, 4)]));
  expectCommute(lvsrUnion, b, c, [const ValueSubkeyRange(low: 2, high: 4)]);

  const d = [ValueSubkeyRange(low: 0, high: 3)];

  expectCommute(lvsrIntersect, a, b, <ValueSubkeyRange>[]);
  expectCommute(lvsrIntersect, c, d, [ValueSubkeyRange.single(3)]);
  expectCommute(lvsrIntersect, d, d, d);

  final e = ListValueSubkeyRangeExt.fromIntPairs([(1, 5), (10, 14), (16, 18)]);
  final f = ListValueSubkeyRangeExt.fromIntPairs([(2, 6), (8, 10), (12, 20)]);
  final g = ListValueSubkeyRangeExt.fromIntPairs([(1, 6), (8, 20)]);

  expectCommute(lvsrUnion, e, f, g);

  final h = ListValueSubkeyRangeExt.fromIntPairs(
      [(2, 5), (10, 10), (12, 14), (16, 18)]);
  expectCommute(lvsrIntersect, e, f, h);

  expectCommute<List<ValueSubkeyRange>>(lvsrUnion, [], [], []);
  expectCommute<List<ValueSubkeyRange>>(lvsrIntersect, [], [], []);
}
