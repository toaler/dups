import {useEffect, useRef, useState} from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event'
import "./App.css";
import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import 'react-tabs/style/react-tabs.css';

function App() {

  const endOfLogsRef = useRef(null);

  const [path, setPath] = useState('');
  const [logs, setLogs] = useState([]);
  const [resources, setResources] = useState(0);
  const [directories, setDirectories] = useState(0);
  const [files, setFiles] = useState(0);
  const [size, setSize] = useState(0);

  // Sample initial data
  const [topKFiles, setTopKFiles] = useState([
    // add more objects as needed
  ]);

  // Suppose you might update this data dynamically, for example, fetching from an API
  useEffect(() => {
    // Function to handle incoming log events
    const handleTopKEvent = (event) => {

      console.log(event.payload);

      try {
        const data = JSON.parse(event.payload);
        console.log(data); // Now `data` is a JavaScript object.
        setTopKFiles(data);
      } catch (e) {
        console.error(`Error parsing JSON: ${e}`);
      }
    };

    // Start listening for log events from the Rust side
    const unsubscribe = listen("top-k-event", handleTopKEvent);

    // Cleanup the listener when the component unmounts
    return () => {
      unsubscribe.then((unsub) => unsub());
    };
  }, []); // Empty dependency array means this effect runs once after the initial render


  // Effect to scroll to the bottom whenever logs update
  useEffect(() => {
    endOfLogsRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [logs]); // Dependency array, this effect runs when `logs` changes


  useEffect(() => {
    // Function to handle incoming log events
    const handleLogEvent = (event) => {

      console.log(event.payload);

      try {
        const data = JSON.parse(event.payload);
        console.log(data); // Now `data` is a JavaScript object.
        setLogs((currentLogs) => [...currentLogs, event.payload]);

        // Update resources
        setResources((currentResources) => currentResources + data.resources);

        // Update directories
        setDirectories((currentDirectories) => currentDirectories + data.directories);

        // Update files
        setFiles((currentFiles) => currentFiles + data.files);

        setSize((currentSize) => currentSize + data.size);
      } catch (e) {
        console.error(`Error parsing JSON: ${e}`);
      }
    };

    // Start listening for log events from the Rust side
    const unsubscribe = listen("log-event", handleLogEvent);

    // Cleanup the listener when the component unmounts
    return () => {
      unsubscribe.then((unsub) => unsub());
    };
  }, []);

  async function scanFilesystem(path) {
    try {
      console.log("Scanning for = " + path);
      const result = await invoke('scan_filesystem', { path });
      console.log(result); // Process result
    } catch (error) {
      console.error(error); // Handle error
    }
  }

  const handleScanClick = () => {
    // Reset states
    setResources(0);
    setDirectories(0);
    setFiles(0);

    // Then initiate the scan
    scanFilesystem(path);
  };

  return (
 <Tabs forceRenderTabPanel defaultIndex={0}>
    <TabList>
      <Tab>Storage</Tab>
      <Tab>Compute</Tab>
      <Tab>Memory</Tab>
      <Tab>Network</Tab>
    </TabList>
    <TabPanel>
      <Tabs forceRenderTabPanel>
        <TabList>
          <Tab>Scan</Tab>
          <Tab>Inspections</Tab>
          <Tab>Staging</Tab>
        </TabList>
        <TabPanel>
          <p>Scan filesystem</p>
          {/* Input field for filesystem path */}

          {/* Button to trigger Rust function */}

          <table>
            <tr>
              <td>
                <input
                    type="text"
                    value={path}
                    onChange={(e) => setPath(e.target.value)}
                    placeholder="Enter filesystem path"
                />
                <button onClick={() => handleScanClick(path)}>Scan</button>
              </td>
              <td>Resources</td>
              <td>{Number(resources).toLocaleString()}</td>
              <td>Directories</td>
              <td>{Number(directories).toLocaleString()}</td>
              <td>Files</td>
              <td>{Number(files).toLocaleString()}</td>
              <td>Size</td>
              <td>{Number(size).toLocaleString()}</td>
            </tr>
          </table>


          <div className="log-container" style={{height: '300px', overflowY: 'auto'}}>
            {logs.map((log, index) => (
                <div key={index}>{log}</div>
            ))}
            {/* Invisible div at the end of your logs */}
            <div ref={endOfLogsRef}/>
          </div>
        </TabPanel>
        <TabPanel>
          <p>Inspections enable automatic high-level analysis of storage</p>
          <table>
            <thead>
            <tr>
              <th>Rank</th>
              <th style={{textAlign: "right"}}>Bytes</th>
              {/* Right-align the header */}
              <th style={{textAlign: "left"}}>Path</th>
            </tr>
            </thead>
            <tbody>
            {topKFiles.map((row, index) => (
                <tr key={index}>
                  <td>{row.rank}</td>
                  <td style={{textAlign: "right"}}>{Number(row.bytes).toLocaleString("en-US")}</td>
                  {/* Right-align and format the bytes column */}
                  <td style={{textAlign: "left"}}>{row.path}</td>
                </tr>
            ))}
            </tbody>
          </table>
        </TabPanel>
        <TabPanel>
          <p>The staging view is used for preparing changes to be carried out on the filesystem</p>
        </TabPanel>
      </Tabs>
    </TabPanel>
   <TabPanel>
     <Tabs forceRenderTabPanel>
       <TabList>
         <Tab>Foo</Tab>
       </TabList>
       <TabPanel>
         <p>bar</p>
         <img src="https://upload.wikimedia.org/wikipedia/en/thumb/2/28/Philip_Fry.png/175px-Philip_Fry.png"
              alt="Philip J. Fry"/>
       </TabPanel>
      </Tabs>
    </TabPanel>
  </Tabs>
  );
}

export default App;