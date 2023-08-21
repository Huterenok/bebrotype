import { Providers } from "app-flat/providers";
import { Navbar, Header } from "widgets";

import "app-flat/styles/globals.css";
import "react-toastify/dist/ReactToastify.css";

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <Providers>
          <div className="layout">
            <Navbar />
            <Header />
            <div className="pages_wrapper">{children}</div>
          </div>
        </Providers>
      </body>
    </html>
  );
}
