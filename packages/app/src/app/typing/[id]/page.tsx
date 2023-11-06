import { Metadata } from "next";
import { TypingPage } from "pages-flat";

export const metadata: Metadata = {
  title: "User's texts typing",
};

export default function Page({ params }: { params: { id: number } }) {
  return <TypingPage textId={params.id}></TypingPage>;
}
