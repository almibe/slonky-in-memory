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

  def addStatement(collection: NamedNode, statement: Statement): Task[PersistedStatement] = {
    def addStatement(persistedStatement: PersistedStatement): Task[PersistedStatement] = Task.eval {
      val newStatements = workingCopy.get.get(collection).get.statements.appended(persistedStatement)
      val newWorkingCopy: Map[NamedNode, Collection] = workingCopy.get + (collection -> Collection(newStatements, workingCopy.get.get(collection).get.counter))
      workingCopy.set(newWorkingCopy)
      PersistedStatement(collection, statement, persistedStatement.context)
    }

    for {
      _         <- createCollection(collection)
      context   <- newNode(collection)
      statement <- addStatement(PersistedStatement(collection, statement, context))
    } yield(statement)
  }

  def cancel(): Unit = isOpen.set(false)

  def close(): Unit = {
    if (isOpen.get()) {
      commit()
    }
    isOpen.set(false)
  }

  private def commit(): Unit = {
    data.set(workingCopy.get())
  }

  def createCollection(collection: NamedNode): Task[NamedNode] = {
    Task.eval {
      if (!workingCopy.get().contains(collection)) {
        val newWorkingCopy: Map[NamedNode, Collection] = workingCopy.get() + (collection -> Collection(List(), Atomic(0L)))
        workingCopy.set(newWorkingCopy)
      }
      collection
    }
  }

  def deleteCollection(collection: NamedNode): Task[NamedNode] = {
    Task.eval {
      if (workingCopy.get().contains(collection)) {
        val newWorkingCopy: Map[NamedNode, Collection] = workingCopy.get() - collection
        workingCopy.set(newWorkingCopy)
      }
      collection
    }
  }

  def newNode(collection: NamedNode): Task[AnonymousNode] = {
    def createNewNode(): Task[AnonymousNode] = Task.eval {
      val nextId = workingCopy.get.get(collection).get.counter.get + 1
      val newWorkingCopy: Map[NamedNode, Collection] = workingCopy.get + (collection -> Collection(workingCopy.get.get(collection).get.statements, Atomic(nextId)))
      workingCopy.set(newWorkingCopy)
      AnonymousNode(nextId)
    }

    for {
      _    <- createCollection(collection)
      node <- createNewNode(collection)
    } yield(node)
  }
}
