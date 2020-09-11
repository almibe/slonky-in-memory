/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.ligature.store.inmemory

import dev.ligature._
import cats.effect.Resource
import monix.eval.Task
import monix.reactive.Observable
import monix.execution.atomic._

private final class InMemoryWriteTx(private val data: Atomic[Map[NamedNode, Collection]]) extends WriteTx {
  private val isOpen = Atomic(true)
  private val workingCopy = Atomic(data.get)

  def addStatement(collection: NamedNode, statement: Statement): Task[PersistedStatement] = ???

  def cancel(): Unit = isOpen.set(false)

  def close(): Unit = {
    if (isOpen.get) {
      ???
    } else {
      ???
    }
    isOpen.set(false)
  }

  def createCollection(collection: NamedNode): Task[NamedNode] = {
    ???
    //    Task.eval {
    //      if (!workingCopy.get.contains(collection)) {
    //        workingCopy.set()
    //      }
    //    }
  }

  def deleteCollection(collection: NamedNode): Task[NamedNode] = ???

  def newEntity(collection: NamedNode): Task[AnonymousNode] = ???
}
