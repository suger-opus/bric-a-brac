"use client";

import AccessesCard from "@/components/cards/accesses";
import BookmarksCard from "@/components/cards/bookmarks";
import CheersCard from "@/components/cards/cheers";
import SearchCard from "@/components/cards/search";
import SettingsCard from "@/components/cards/settings";

const Home = () => {
  // const [graphs, setGraphs] = useState<{ id: string; name: string }[]>([]);
  // const [currentGraphId, setCurrentGraphId] = useState<string | undefined>(
  //   undefined
  // );
  // const [graphData, setGraphData] = useState<GraphData>({
  //   nodes: [],
  //   edges: [],
  // });
  // const [isLoading, setIsLoading] = useState(false);

  // const loadGraph = async () => {
  //   try {
  //     if (!currentGraphId) return;
  //     setIsLoading(true);
  //     const data = await searchGraph(currentGraphId);
  //     setGraphData(data);
  //   } catch (error) {
  //     console.error("Failed to load graph:", error);
  //   } finally {
  //     setIsLoading(false);
  //   }
  // };

  // useEffect(() => {
  //   loadGraph();
  // }, [currentGraphId]);

  // const handleGraphSelect = (graphId: string) => {
  //   setCurrentGraphId(graphId);
  // };

  // const handleGraphCreate = (name: string) => {
  //   const newGraphId = crypto.randomUUID();
  //   setGraphs([...graphs, { id: newGraphId, name }]);
  //   setCurrentGraphId(newGraphId);
  // };

  return (
    <div className="mx-40 my-20">
      <div className="grid grid-cols-2 gap-4 p-4">
        <SearchCard />
        <BookmarksCard />
        <div className="col-span-2">
          <AccessesCard />
        </div>
        <CheersCard />
        <SettingsCard />
      </div>
    </div>
  );
};

export default Home;
