/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.almibe.slonky.inmemory

import cats.effect.{IO, Resource}
import dev.almibe.slonky.{Slonky, SlonkyInstance}

final class InMemorySlonky extends Slonky {
  private val acquire: IO[InMemorySlonkyInstance] = IO(new InMemorySlonkyInstance())

  private def release(session: InMemorySlonkyInstance): IO[Unit] = {
    IO { session.close() }
  }

  override def instance: Resource[IO, SlonkyInstance] = {
    Resource.make(acquire)(release)
  }
}
