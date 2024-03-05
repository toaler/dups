import {useEffect, useRef, useState} from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event'
import "./App.css";
import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import 'react-tabs/style/react-tabs.css';

function App() {

  // State to hold the filesystem path input
  const [path, setPath] = useState('');
  const [logs, setLogs] = useState([]);

  // Create a ref for the log container
  const endOfLogsRef = useRef(null);
  const [resources, setResources] = useState(0);
  const [directories, setDirectories] = useState(0);
  const [files, setFiles] = useState(0);

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
              <td>{resources.toLocaleString()}</td>
              <td>Directories</td>
              <td>{directories.toLocaleString()}</td>
              <td>Files</td>
              <td>{files.toLocaleString()}</td>
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
          <img src="https://upload.wikimedia.org/wikipedia/en/thumb/2/28/Philip_Fry.png/175px-Philip_Fry.png" alt="Philip J. Fry" />
        </TabPanel>
      </Tabs>
    </TabPanel>
  </Tabs>
  );
}

export default App;
