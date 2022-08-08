// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

pub use crate::{
    psp34,
    psp34::balances,
    traits::psp34::{
        extensions::mintable::*,
        *,
    },
};
pub use psp34::{
    Internal as _,
    Transfer as _,
};

use openbrush::traits::{
    AccountId,
    OccupiedStorage,
    Storage,
};

impl<B, T> PSP34Mintable for T
where
    B: balances::BalancesManager,
    T: Storage<psp34::Data<B>>,
    T: OccupiedStorage<{ psp34::STORAGE_KEY }, WithData = psp34::Data<B>>,
{
    default fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
        self._mint_to(account, id)
    }
}