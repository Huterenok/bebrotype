import { Metadata } from "next";
import { TextPage } from "pages-flat";

export const metadata: Metadata = {
  title: "User's Texts",
};

export default function Page({ params }: { params: { id: number } }) {
  return <TextPage id={params.id} />;
}
