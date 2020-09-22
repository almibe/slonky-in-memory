/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature.store.inmemory

import dev.ligature._
import cats.effect.Resource
import monix.eval.Task
import monix.reactive.Observable
import monix.execution.atomic._

private final class InMemoryReadTx(private val data: Atomic[Map[NamedNode, Collection]]) extends ReadTx {
  def allStatements(collection: NamedNode): Observable[PersistedStatement] = {
    val col: Option[Collection] = data.get().get(collection)
    if (col.isDefined) {
      Observable.fromIterable(col.get.statements)
    } else {
      Observable.empty
    }
  }

  def collections(): Observable[NamedNode] = Observable.fromIterable(data.get.map { v => v._1 })

  def collections(prefix: NamedNode): Observable[NamedNode] = ???

  def collections(from: NamedNode, to: NamedNode): Observable[NamedNode] = ???

  def matchStatements(collection: NamedNode,
                      subject: Option[Node],
                      predicate: Option[NamedNode],
                      `object`: Option[Object]): Observable[PersistedStatement] = ???

  def statementByContext(collection: NamedNode, context: AnonymousNode): Task[Option[PersistedStatement]] = ???
}
