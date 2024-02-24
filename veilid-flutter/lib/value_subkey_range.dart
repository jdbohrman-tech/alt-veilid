import 'dart:math';

import 'package:freezed_annotation/freezed_annotation.dart';

part 'value_subkey_range.freezed.dart';
part 'value_subkey_range.g.dart';

@freezed
class ValueSubkeyRange with _$ValueSubkeyRange {
  @Assert('low >= 0 && low <= high', 'range is invalid')
  const factory ValueSubkeyRange({
    required int low,
    required int high,
  }) = _ValueSubkeyRange;

  factory ValueSubkeyRange.single(int val) =>
      ValueSubkeyRange(low: val, high: val);

  factory ValueSubkeyRange.fromJson(dynamic json) =>
      _$ValueSubkeyRangeFromJson(json as Map<String, dynamic>);
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
}

extension ListValueSubkeyRangeExt on List<ValueSubkeyRange> {
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
}
