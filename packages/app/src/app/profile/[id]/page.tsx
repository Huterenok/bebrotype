import { ProfilePage } from "pages-flat";

export default function Page({ params }: { params: { id: number } }) {
  return <ProfilePage id={params.id}/>;
}
