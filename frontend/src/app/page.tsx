import dynamic from "next/dynamic";

// NOTE: This lazy loading avoids window object reference errors that occur when next.js renders
// client components on the server side(i.e., at`next build`)
const Converter = dynamic(() => import("../ui/converter"), {
  loading: () => <p>Loading...</p>,
  ssr: false,
});

/**
 * # Root page (URL: /).
 */
export default function Home() {
  return <Converter />;
}
