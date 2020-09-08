/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature.store.inmemory

import dev.ligature._
import cats.effect.Resource
import monix.eval.Task
import monix.reactive.Observable

final class InMemoryLigature extends Ligature {
  def session(): Resource[Task, LigatureSession] = {
    ???
  }
}

private final class InMemoryLigatureSession extends LigatureSession {
  def read(): Resource[Task, ReadTx] = {
    ???
  }

  def write(): Resource[Task, WriteTx] = {
    ???
  }
}

private final class InMemoryReadTx extends ReadTx {
  def allStatements(collection: LocalNode): Observable[PersistedStatement] = ???
  def collections(): Observable[LocalNode] = ???
  def collections(prefix: LocalNode): Observable[LocalNode] = ???
  def collections(from: LocalNode, to: LocalNode): Observable[LocalNode] = ???
  def matchStatements(collection: LocalNode,
                      subject: Option[Node],
                      predicate: Option[NamedNode],
                      `object`: Option[Object]): Observable[PersistedStatement] = ???
  def statementByContext(collection: LocalNode, context: AnonymousNode): Task[Option[PersistedStatement]] = ???
}

private final class InMemoryWriteTx extends WriteTx {
  def addStatement(collection: LocalNode, statement: Statement): Task[PersistedStatement] = ???
    def cancel(): Unit = ???
    def createCollection(collection: LocalNode): Task[LocalNode] = ???
    def deleteCollection(collection: LocalNode): Task[LocalNode] = ???
    def newEntity(collection: LocalNode): Task[AnonymousNode] = ???
}
