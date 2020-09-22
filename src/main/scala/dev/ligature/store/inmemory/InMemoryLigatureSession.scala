/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature.store.inmemory

import dev.ligature._
import cats.effect.Resource
import monix.eval.Task
import monix.reactive.Observable
import monix.execution.atomic._

private case class Collection(statements: List[PersistedStatement], counter: Atomic[Long])

private final class InMemoryLigatureSession extends LigatureSession {
  private val data: Atomic[Map[NamedNode, Collection]] = Atomic(Map[NamedNode, Collection]())

  def close(): Unit = {
    data.set(Map[NamedNode, Collection]())
  }

  private val startReadTx: Task[InMemoryReadTx] = {
    Task.eval { new InMemoryReadTx(data) }
  }

  private def releaseReadTx(tx: InMemoryReadTx): Task[Unit] = {
    Task.unit
  }

  def read(): Resource[Task, ReadTx] = {
    Resource.make(startReadTx)(releaseReadTx)
  }

  private val startWriteTx: Task[InMemoryWriteTx] = {
    Task.eval { new InMemoryWriteTx(data) }
  }

  private def releaseWriteTx(tx: InMemoryWriteTx): Task[Unit] = {
    Task.eval { tx.close() } //close double checks if transaction has been canceled
  }

  def write(): Resource[Task, WriteTx] = {
    Resource.make(startWriteTx)(releaseWriteTx)
  }
}
