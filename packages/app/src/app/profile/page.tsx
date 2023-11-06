import { Metadata } from "next";

import { ProfileMePage } from "pages-flat";
import { useAuth } from "features/Auth";

export const metadata: Metadata = {
  title: "Profile",
  icons: "/icons/banana.svg",
};

export default function Page() {
  const user = useAuth();

  return <ProfileMePage id={user.id} />;
}
