/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.almibe.slonky.inmemory

import java.util.concurrent.atomic.AtomicReference

import cats.effect.IO
import dev.almibe.slonky.SlonkyReadTx
import scodec.bits.ByteVector
import fs2.Stream

private final class InMemoryReadTx(private val data: AtomicReference[Map[ByteVector, ByteVector]]) extends SlonkyReadTx {
  override def keyExists(key: ByteVector): IO[Boolean] = ???

  override def prefixExists(prefix: ByteVector): IO[Boolean] = ???

  override def get(key: ByteVector): IO[Option[ByteVector]] = ???

  override def prefixScan(prefix: ByteVector): Stream[IO, (ByteVector, ByteVector)] = ???

  override def rangeScan(from: ByteVector, to: ByteVector): Stream[IO, (ByteVector, ByteVector)] = ???

  override def scanAll(): Stream[IO, (ByteVector, ByteVector)] = ???

  def cancel(): IO[Unit] = IO.unit
}
