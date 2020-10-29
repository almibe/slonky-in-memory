/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.almibe.slonky.inmemory

import java.util.concurrent.atomic.{AtomicBoolean, AtomicReference}

import cats.effect.IO
import dev.almibe.slonky.SlonkyWriteTx
import scodec.bits.ByteVector

import scala.collection.immutable.SortedMap

private final class InMemoryWriteTx(private val data: AtomicReference[SortedMap[ByteVector, ByteVector]]) extends SlonkyWriteTx {
  private val isOpen = new AtomicBoolean(true)
  private val workingCopy = new AtomicReference(data.get)

  def cancel(): IO[Unit] = IO { isOpen.set(false) }

  def close(): Unit = {
    if (isOpen.get()) {
      commit()
    }
    isOpen.set(false)
  }

  private def commit(): Unit = {
    data.set(workingCopy.get())
  }

  override def keyExists(key: ByteVector): IO[Boolean] = ???

  override def prefixExists(prefix: ByteVector): IO[Boolean] = ???

  override def get(key: ByteVector): IO[Option[ByteVector]] = ???

  override def put(key: ByteVector, value: ByteVector): IO[Unit] = IO {
      val newWorkingCopy = workingCopy.get() + (key -> value)
      workingCopy.set(newWorkingCopy)
    }

  override def remove(key: ByteVector): IO[Unit] = IO {
    val newWorkingCopy = workingCopy.get().removed(key)
    workingCopy.set(newWorkingCopy)
  }
}
