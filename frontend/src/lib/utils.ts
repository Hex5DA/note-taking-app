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
  let response = await fetch(`${URL}/api/notes`, { method: "GET" });
  let notes: Note[] = await response.json();
  return notes;
}

export function updateNote(id: number, note: OptionalNote) {
  return fetch(`${URL}/api/notes/${id}`, {
    method: "PATCH",
    body: JSON.stringify(note),
    headers: {
      "Content-type": "application/json; charset=UTF-8",
    },
  })
    .then((response) => response.json())
    .then((json) => json);
}

export function deleteNote(id: number) {
  return fetch(`${URL}/api/notes/${id}`, { method: "DELETE" }).then(
    (response) => response.ok
  );
}

export function createNote(note: Note) {
  return fetch(`${URL}/api/notes/create`, {
    method: "POST",
    body: JSON.stringify(note),
    headers: {
      "Content-type": "application/json; charset=UTF-8",
    },
  }).then((response) => response.statusText);
}
