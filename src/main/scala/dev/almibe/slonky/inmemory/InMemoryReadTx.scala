/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.almibe.slonky.inmemory

import cats.effect.IO
import dev.almibe.slonky.SlonkyReadTx
import scodec.bits.ByteVector
import fs2.Stream
import scala.collection.immutable.SortedMap

private final class InMemoryReadTx(private val data: SortedMap[ByteVector, ByteVector]) extends SlonkyReadTx {
  override def keyExists(key: ByteVector): IO[Boolean] = SharedLookup.keyExists(data, key)

  override def prefixExists(prefix: ByteVector): IO[Boolean] = SharedLookup.prefixExists(data, prefix)

  override def get(key: ByteVector): IO[Option[ByteVector]] = SharedLookup.get(data, key)

  override def prefixScan(prefix: ByteVector): Stream[IO, (ByteVector, ByteVector)] =
    Stream.fromIterator[IO] {
      val range = data.rangeFrom(prefix)
      if (range.isEmpty) {
        Iterator.empty
      } else {
        range.takeWhile { x => x._1.startsWith(prefix) }.iterator
      }
    }

  override def rangeScan(from: ByteVector, to: ByteVector): Stream[IO, (ByteVector, ByteVector)] =
    Stream.fromIterator[IO](data.range(from, to).iterator)

  override def scanAll(): Stream[IO, (ByteVector, ByteVector)] = Stream.fromIterator[IO](data.iterator)

  def cancel(): IO[Unit] = IO.unit
}
