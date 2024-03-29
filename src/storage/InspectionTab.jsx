import React, {useEffect, useState} from 'react';
import {listen} from "@tauri-apps/api/event";

import Tooltip from '@mui/material/Tooltip';
import FolderZipIcon from '@mui/icons-material/FolderZip';
import CompressIcon from '@mui/icons-material/Compress';
import DeleteIcon from '@mui/icons-material/Delete';
import TextSnippetIcon from '@mui/icons-material/TextSnippet';
import NotListedLocationIcon from '@mui/icons-material/NotListedLocation';
import ArchiveIcon from '@mui/icons-material/Archive';
import CodeIcon from '@mui/icons-material/Code';
import HtmlIcon from '@mui/icons-material/Html';

function getMimeTypeIcon(compressible, mime_type) {
    if (compressible === "-1") {
        return (
            <Tooltip title={mime_type} placement="right">
                <FolderZipIcon/>
            </Tooltip>
        );
    }

    if (mime_type.startsWith('text/')) {
        return (
            <Tooltip title={mime_type} placement="right">
                <TextSnippetIcon/>
            </Tooltip>
        );
    }

    if (mime_type.startsWith('image/')) {
        return (
            <Tooltip title={mime_type} placement="right">
                <ImageIcon/>;
            </Tooltip>
        );
    }

    switch (mime_type) {
        case 'application/octet-stream':
            return (
                <Tooltip title={mime_type} placement="right">
                    <NotListedLocationIcon/>
                </Tooltip>
            );
        case 'application/x-tar':
            return (
                <Tooltip title={mime_type}>
                    <ArchiveIcon/>
                </Tooltip>
            );
        default:
            if (mime_type.startsWith('application/')) {
                return (
                    <Tooltip title={mime_type} placement="right">
                        <CodeIcon/>
                    </Tooltip>
                );
            }
            return mime_type;
    }
}

function InspectionTab({setSelectedRows}) {

    const [topKFiles, setTopKFiles] = useState([]);

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

    const handleCheckboxChange = (event) => {
        console.log(event);
        const value = event.target.value;
        const isChecked = event.target.checked;

        // Update the state based on whether the checkbox was checked or unchecked
        if (isChecked) {
            setSelectedRows(prev => [...prev, value]);
        } else {
            setSelectedRows(prev => prev.filter(row => row !== value));
        }
    };

    return (<div>
        <style>
            {`
                table, th, td {
                    border: 2px solid grey; /* Adjust the border size and color as needed */
                    border-collapse: collapse; /* Removes the space between borders */
                }
                tr:hover {
                    background-color: #006abc; /* Light grey background on hover */
                }
            `}
        </style>
        <table style={{borderCollapse: "collapse"}}>
            <thead>
            <tr>
                <th>Stage</th>
                <th style={{textAlign: "center"}}>Action</th>
                <th>Rank</th>
                <th style={{textAlign: "right"}}>Bytes</th>
                <th>
                    <span style={{display: "block", textAlign: "right"}}>Last</span>
                    <span style={{display: "block", textAlign: "right"}}>Write</span>
                </th>
                <th>
                    <span style={{display: "block", textAlign: "right"}}>Last</span>
                    <span style={{display: "block", textAlign: "right"}}>Read</span>
                </th>
                <th>
                    <span style={{display: "block", textAlign: "right"}}>Last</span>
                    <span style={{display: "block", textAlign: "right"}}>Write</span>
                    <span style={{display: "block", textAlign: "right"}}>Days</span>
                </th>
                <th>
                    <span style={{display: "block", textAlign: "right"}}>Last</span>
                    <span style={{display: "block", textAlign: "right"}}>Read</span>
                    <span style={{display: "block", textAlign: "right"}}>Days</span>
                </th>
                <th style={{textAlign: "left"}}>Type</th>
                <th style={{textAlign: "left"}}>Path</th>
            </tr>
            </thead>
            <tbody>
            {topKFiles.map((row, index) => (<tr key={index}>
                <td>
                    <input type="checkbox" value={row.path} onChange={handleCheckboxChange}/>
                </td>
                <td style={{textAlign: "center"}}><DeleteIcon/>{row.compressible === "1" ? <CompressIcon/> : null}</td>
                <td style={{textAlign: "center"}}>{row.rank}</td>
                <td style={{textAlign: "right"}}>{Number(row.bytes).toLocaleString("en-US")}</td>

                <td style={{textAlign: "right"}}>{row.modified}</td>
                <td style={{textAlign: "right"}}>{row.accessed}</td>
                <td style={{textAlign: "right"}}>{row.modified_days}</td>
                <td style={{textAlign: "right"}}>{row.accessed_days}</td>
                <td style={{textAlign: "center"}}>{getMimeTypeIcon(row.compressible, row.mime_type)}</td>
                <td style={{textAlign: "left"}}>{row.path}</td>
            </tr>))}
            </tbody>
        </table>
    </div>);
}

export default InspectionTab;