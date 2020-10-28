/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.almibe.slonky.inmemory

import dev.ligature._
import cats.effect.Resource
import monix.eval.Task

final class InMemoryLigature extends Ligature {
  private val acquire: Task[InMemoryLigatureSession] = Task.eval { new InMemoryLigatureSession() }

  private def release(session: InMemoryLigatureSession): Task[Unit] = {
    Task.eval { session.close() }
  }

  def session(): Resource[Task, LigatureSession] = {
    Resource.make(acquire)(release)
  }
}
