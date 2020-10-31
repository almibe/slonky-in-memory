/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.almibe.slonky.inmemory

import cats.effect.IO
import scodec.bits.ByteVector
import scala.collection.SortedMap

object SharedLookup {
  def keyExists(data: SortedMap[ByteVector, ByteVector], key: ByteVector): IO[Boolean] = IO {
    data.contains(key)
  }

  def prefixExists(data: SortedMap[ByteVector, ByteVector], prefix: ByteVector): IO[Boolean] = IO {
    val range = data.rangeFrom(prefix)
    if (range.isEmpty) {
      false
    } else {
      range.firstKey.startsWith(prefix)
    }
  }

  def get(data: SortedMap[ByteVector, ByteVector], key: ByteVector): IO[Option[ByteVector]] = IO {
    data.get(key)
  }
}
