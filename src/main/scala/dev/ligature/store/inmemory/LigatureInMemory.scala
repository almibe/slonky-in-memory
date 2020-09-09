/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature.store.inmemory

import dev.ligature._
import cats.effect.Resource
import monix.eval.Task
import monix.reactive.Observable
import monix.execution.atomic._

final class InMemoryLigature extends Ligature {
  private val acquire: Task[InMemoryLigatureSession] = Task.eval { new InMemoryLigatureSession() }

  private def release(session: InMemoryLigatureSession): Task[Unit] = {
    Task.eval { session.close() }
  }

  def session(): Resource[Task, LigatureSession] = {
    Resource.make(acquire)(release)
  }
}

private case class Collection(statements: List[Statement], counter: Atomic[Long])

private final class InMemoryLigatureSession extends LigatureSession {
  private val data: Atomic[Map[String, Collection]] = Atomic(Map[String, Collection]())

  def close(): Unit = {
    data.set(Map[String, Collection]())
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

private final class InMemoryReadTx(private val data: Atomic[Map[String, Collection]]) extends ReadTx {
  def allStatements(collection: LocalNode): Observable[PersistedStatement] = ???

  def collections(): Observable[LocalNode] = Observable.fromIterable(data.get.map { v => LocalNode(v._1) })

  def collections(prefix: LocalNode): Observable[LocalNode] = ???

  def collections(from: LocalNode, to: LocalNode): Observable[LocalNode] = ???

  def matchStatements(collection: LocalNode,
                      subject: Option[Node],
                      predicate: Option[NamedNode],
                      `object`: Option[Object]): Observable[PersistedStatement] = ???

  def statementByContext(collection: LocalNode, context: AnonymousNode): Task[Option[PersistedStatement]] = ???
}

private final class InMemoryWriteTx(private val data: Atomic[Map[String, Collection]]) extends WriteTx {
  private val isOpen = Atomic(true)

  def addStatement(collection: LocalNode, statement: Statement): Task[PersistedStatement] = ???

  def cancel(): Unit = ???

  def close(): Unit = ???

  def createCollection(collection: LocalNode): Task[LocalNode] = ???

  def deleteCollection(collection: LocalNode): Task[LocalNode] = ???

  def newEntity(collection: LocalNode): Task[AnonymousNode] = ???
}
