/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package dev.almibe.slonky.inmemory

import dev.almibe.slonky.Slonky
import dev.almibe.slonky.test.SlonkySuite

class LigatureInMemorySuite extends SlonkySuite {
  override def createSlonky: Slonky = {
    new InMemorySlonky()
  }
}
