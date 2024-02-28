import 'dart:math';

import 'package:equatable/equatable.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

@immutable
class ValueSubkeyRange extends Equatable {
  const ValueSubkeyRange({
    required this.low,
    required this.high,
  }) : assert(low >= 0 && low <= high, 'range is invalid');

  factory ValueSubkeyRange.single(int val) =>
      ValueSubkeyRange(low: val, high: val);
  factory ValueSubkeyRange.make(int low, int high) =>
      ValueSubkeyRange(low: low, high: high);
  factory ValueSubkeyRange.fromIntPair((int, int) pair) =>
      ValueSubkeyRange(low: pair.$1, high: pair.$2);
  factory ValueSubkeyRange.fromIntList(List<int> intlist) {
    assert(intlist.length == 2, 'range must be a two item list');
    return ValueSubkeyRange(low: intlist[0], high: intlist[1]);
  }
  factory ValueSubkeyRange.fromJson(dynamic json) =>
      ValueSubkeyRange.fromIntList((json as List<dynamic>).cast<int>());

  List<int> toJson() => <int>[low, high];

  @override
  List<Object> get props => [low, high];

  final int low;
  final int high;
}

extension ValueSubkeyRangeExt on ValueSubkeyRange {
  bool contains(int v) => low <= v && v <= high;
  List<ValueSubkeyRange> remove(int v) {
    if (v < low || v > high) {
      return [ValueSubkeyRange(low: low, high: high)];
    }
    if (v == low) {
      if (v == high) {
        return [];
      } else {
        return [ValueSubkeyRange(low: v + 1, high: high)];
      }
    } else if (v == high) {
      return [ValueSubkeyRange(low: low, high: v - 1)];
    } else {
      return [
        ValueSubkeyRange(low: low, high: v - 1),
        ValueSubkeyRange(low: v + 1, high: high)
      ];
    }
  }

  ValueSubkeyRange? intersect(ValueSubkeyRange other) {
    if (high < other.low || low > other.high) {
      return null;
    }
    return ValueSubkeyRange(
        low: max(low, other.low), high: min(high, other.high));
  }

  ValueSubkeyRange? union(ValueSubkeyRange other) {
    if (high < (other.low - 1) || low > (other.high + 1)) {
      return null;
    }
    return ValueSubkeyRange(
        low: min(low, other.low), high: max(high, other.high));
  }
}

extension ListValueSubkeyRangeExt on List<ValueSubkeyRange> {
  static List<ValueSubkeyRange> fromIntPairs(List<(int, int)> x) =>
      x.map(ValueSubkeyRange.fromIntPair).toList();

  void validate() {
    int? lastHigh;
    for (final r in this) {
      assert(lastHigh == null || r.low > lastHigh,
          'subrange not in order or disjoint');
      lastHigh = r.high;
    }
  }

  bool containsSubkey(int v) => indexWhere((e) => e.contains(v)) != -1;
  List<ValueSubkeyRange> removeSubkey(int v) {
    for (var i = 0; i < length; i++) {
      if (this[i].contains(v)) {
        return [...sublist(0, i), ...this[i].remove(v), ...sublist(i + 1)];
      }
    }
    return toList();
  }

  int? get firstSubkey => isNotEmpty ? first.low : null;

  List<ValueSubkeyRange> intersectSubkeys(List<ValueSubkeyRange> other) {
    final out = <ValueSubkeyRange>[];
    for (var i = 0, j = 0; i < length && j < other.length;) {
      final vsrThis = this[i];
      final vsrOther = other[j];
      if (vsrThis.high < vsrOther.low) {
        i++;
        continue;
      }
      if (vsrOther.high < vsrThis.low) {
        j++;
        continue;
      }

      // Otherwise we intersect
      out.add(vsrThis.intersect(vsrOther)!);

      // Iterate whichever has a lower high
      // If they both have the same high then both ranges are exhausted
      // and should be iterated
      if (vsrThis.high < vsrOther.high) {
        // Iterate this because other could still have some overlaps
        i++;
      } else if (vsrThis.high == vsrOther.high) {
        // Iterate both because both ranges are exhausted
        i++;
        j++;
      } else {
        // Iterate other because this could still have some overlaps
        j++;
      }
    }
    return out;
  }

  List<ValueSubkeyRange> unionSubkeys(List<ValueSubkeyRange> other) {
    final out = <ValueSubkeyRange>[];
    ValueSubkeyRange? current;
    for (var i = 0, j = 0; i < length || j < other.length;) {
      if (i == length) {
        current = other[j];
        j++;
      } else if (j == other.length) {
        current = this[i];
        i++;
      } else if (this[i].low < other[j].low) {
        current = this[i];
        i++;
      } else {
        current = other[j];
        j++;
      }

      if (out.isNotEmpty && out.last.high >= (current.low - 1)) {
        out[out.length - 1] = ValueSubkeyRange(
            low: out.last.low, high: max(out.last.high, current.high));
      } else {
        out.add(current);
      }
    }
    return out;
  }
}
