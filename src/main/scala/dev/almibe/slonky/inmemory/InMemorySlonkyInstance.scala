/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.almibe.slonky.inmemory

import java.util.concurrent.atomic.AtomicReference

import cats.effect.{IO, Resource}
import dev.almibe.slonky.{SlonkyReadTx, SlonkyInstance, SlonkyWriteTx}
import scodec.bits.ByteVector

private final class InMemorySlonkyInstance extends SlonkyInstance {
  private val data: AtomicReference[Map[ByteVector, ByteVector]] = new AtomicReference(Map[ByteVector, ByteVector]())

  def close(): Unit = {
    data.set(Map[ByteVector, ByteVector]())
  }

  private val startReadTx: IO[InMemoryReadTx] = {
    IO { new InMemoryReadTx(data) }
  }

  private def releaseReadTx(tx: InMemoryReadTx): IO[Unit] = {
    tx.cancel()
  }

  def read: Resource[IO, SlonkyReadTx] = {
    Resource.make(startReadTx)(releaseReadTx)
  }

  private val startWriteTx: IO[InMemoryWriteTx] = {
    IO { new InMemoryWriteTx(data) }
  }

  private def releaseWriteTx(tx: InMemoryWriteTx): IO[Unit] = {
    IO { tx.close() } //close double checks if transaction has been canceled
  }

  def write: Resource[IO, SlonkyWriteTx] = {
    Resource.make(startWriteTx)(releaseWriteTx)
  }
}
