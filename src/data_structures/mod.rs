// Copyright © 2021 Hovig Manjikian
//
// This file is part of json.
//
// json is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// json is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with json.  If not, see <https://www.gnu.org/licenses/>.

mod j_string;
mod j_whitespace;
mod j_number;
mod j_value;
mod j_object;

pub use j_string::JString;
pub use j_whitespace::JWhitespace;
pub use j_number::JNumber;
pub use j_value::JValue;
pub use j_object::JObject;


trait Serialize {
    /// Creates a serialization of the data-structure as a JSON string with minimal or
    /// no whitespace characters.
    fn serialize(&self) -> String;
}
