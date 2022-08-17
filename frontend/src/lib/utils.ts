import type { Note, OptionalNote } from "@/lib/types";

const URL = "http://0.0.0.0:8080";
const MAX_TEXT = 135;

export function format_description(description: string): string {
  if (description.length > MAX_TEXT) {
    return description.slice(0, MAX_TEXT - 3) + "...";
  }

  return description;
}

export async function fetchNotes() {
  console.log("Fetching data!");
  let response = await fetch(`${URL}/api/notes`, { method: "get" });
  let notes: Note[] = await response.json();
  return notes;
}

export function updateNote(id: number, note: OptionalNote) {
  let temp = fetch(`${URL}/api/notes/${id}`, {
    method: "patch",
    body: JSON.stringify(note),
    headers: {
        'Content-type': 'application/json; charset=UTF-8',
    },
  })
    .then((response) => response.json())
    .then((json) => json);

  console.log(temp);
  return temp;
}
